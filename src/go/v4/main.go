package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"runtime"
	"sort"
	"sync"
	"sync/atomic"
	"time"
)

// v4: share-nothing parallel workers over pread, work-stealing chunks.
//
// v2/v3 bottlenecks: a single scanner goroutine feeding workers, one string
// allocation per line, one []string allocation per split, and a shared
// mutex/channel on the reduce side.
//
// Design notes (measured on Apple M2, macOS):
//   - pread per worker, NOT mmap: macOS serializes concurrent page faults on
//     the same mapped file behind a per-VM-object lock, which caps an mmap
//     version of this exact code at ~1 core (~6.5s). read() copies from the
//     page cache in parallel just fine.
//   - work-stealing 64MB chunks instead of one static chunk per core: the M2
//     has 4 performance + 4 efficiency cores, so equal static splits finish
//     whenever the slowest E-core does; stealing keeps all cores busy to the
//     end.
//   - zero allocations per line: fields located with bytes.IndexByte (SIMD),
//     months counted in a flat per-worker array indexed by (year*12+month),
//     names counted in a per-worker open-addressing table whose keys live in
//     an arena. Merge happens once at the end.

const (
	tblBits = 21
	tblSize = 1 << tblBits
	tblMask = tblSize - 1

	monthSlots = 1024
	monthBase  = 1990 // idx = (year-monthBase)*12 + month-1

	chunkSize = 128 << 20
	bufSize   = 16 << 20
)

var commaSpace = []byte(", ")

type slot struct {
	hash uint64
	off  int32
	nlen int32
	cnt  int64
}

// nameTable is a linear-probing table; keys live in one contiguous arena so
// inserts allocate only on arena growth (amortized) and lookups touch one
// slot then the key bytes. Sized so the distinct before-comma keys in this
// data stay well under 50% load; it does not grow.
type nameTable struct {
	slots []slot
	arena []byte
}

func newNameTable() *nameTable {
	return &nameTable{slots: make([]slot, tblSize), arena: make([]byte, 0, 1<<22)}
}

func hashBytes(b []byte) uint64 {
	h := uint64(0x9E3779B97F4A7C15)
	for len(b) >= 8 {
		h = (h ^ binary.LittleEndian.Uint64(b)) * 0xA24BAED4963EE407
		b = b[8:]
	}
	if len(b) > 0 {
		var tail uint64
		for i := len(b) - 1; i >= 0; i-- {
			tail = tail<<8 | uint64(b[i])
		}
		h = (h ^ tail) * 0x9FB21C651E98DF25
	}
	return h ^ h>>29
}

func (t *nameTable) inc(name []byte) {
	h := hashBytes(name)
	i := h & tblMask
	for {
		s := &t.slots[i]
		if s.cnt == 0 {
			s.hash = h
			s.off = int32(len(t.arena))
			s.nlen = int32(len(name))
			s.cnt = 1
			t.arena = append(t.arena, name...)
			return
		}
		if s.hash == h && s.nlen == int32(len(name)) && bytes.Equal(t.arena[s.off:s.off+s.nlen], name) {
			s.cnt++
			return
		}
		i = (i + 1) & tblMask
	}
}

type result struct {
	lines    int64
	months   [monthSlots]int64
	spill    map[string]int64
	names    *nameTable
	name433  string
	name4324 string // line 43244
}

func (r *result) processLine(line []byte, capture bool) {
	r.lines++
	p := 0
	for k := 0; k < 4; k++ { // skip fields 0-3
		j := bytes.IndexByte(line[p:], '|')
		if j < 0 {
			return
		}
		p += j + 1
	}
	ds := p
	j := bytes.IndexByte(line[p:], '|')
	if j < 0 {
		return
	}
	de := p + j
	p = de + 1
	for k := 0; k < 2; k++ { // skip fields 5-6
		j = bytes.IndexByte(line[p:], '|')
		if j < 0 {
			return
		}
		p += j + 1
	}
	ns := p
	j = bytes.IndexByte(line[p:], '|')
	if j < 0 {
		return
	}
	ne := p + j

	date := line[ds:de]
	name := line[ns:ne]

	if len(date) >= 6 &&
		date[0]-'0' < 10 && date[1]-'0' < 10 && date[2]-'0' < 10 &&
		date[3]-'0' < 10 && date[4]-'0' < 10 && date[5]-'0' < 10 {
		y := int(date[0]-'0')*1000 + int(date[1]-'0')*100 + int(date[2]-'0')*10 + int(date[3]-'0')
		m := int(date[4]-'0')*10 + int(date[5]-'0')
		idx := (y-monthBase)*12 + m - 1
		if uint(idx) < monthSlots {
			r.months[idx]++
		} else {
			r.spill[string(date[:6])]++
		}
	} else if len(date) >= 6 {
		r.spill[string(date[:6])]++
	} else {
		r.spill[string(date)]++
	}

	// same rule as v1-v3: gate on ", " anywhere, key = before FIRST comma,
	// skip empty keys
	if ci := bytes.IndexByte(name, ','); ci > 0 {
		matched := ci+1 < len(name) && name[ci+1] == ' '
		if !matched && ci+1 < len(name) {
			matched = bytes.Index(name[ci+1:], commaSpace) >= 0
		}
		if matched {
			r.names.inc(name[:ci])
		}
	}

	if capture {
		if r.lines == 433 {
			r.name433 = string(name)
		} else if r.lines == 43244 {
			r.name4324 = string(name)
		}
	}
}

// processChunk handles every line that STARTS in [start,end); the final line
// may extend past end and is read to completion. When start > 0 the bytes up
// to the first newline belong to the previous chunk and are skipped.
// capture is set only for chunk 0, whose local line numbers are global.
func processChunk(f *os.File, buf []byte, start, end int64, capture bool, r *result) {
	bufOff := start
	carry := 0
	aligned := start == 0
	eof := false

	for {
		n, err := f.ReadAt(buf[carry:], bufOff+int64(carry))
		valid := carry + n
		if err == io.EOF {
			eof = true
		} else if err != nil && n == 0 {
			return
		}
		if valid == 0 {
			return
		}

		p := 0
		if !aligned {
			j := bytes.IndexByte(buf[:valid], '\n')
			if j < 0 {
				if eof {
					return
				}
				bufOff += int64(valid)
				carry = 0
				continue
			}
			p = j + 1
			aligned = true
			if bufOff+int64(p) >= end {
				return
			}
		}

		for p < valid {
			if bufOff+int64(p) >= end {
				return
			}
			nl := bytes.IndexByte(buf[p:valid], '\n')
			if nl < 0 {
				if eof { // final line without trailing newline
					r.processLine(buf[p:valid], capture)
					return
				}
				break
			}
			r.processLine(buf[p:p+nl], capture)
			p += nl + 1
		}
		if eof && p >= valid {
			return
		}
		carry = valid - p
		if carry == valid && valid == len(buf) {
			return // single line larger than buffer; not this data
		}
		copy(buf, buf[p:valid])
		bufOff += int64(p)
	}
}

func main() {
	startTime := time.Now()
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: v4 <file>")
		os.Exit(1)
	}
	path := os.Args[1]
	fmt.Println("processing file:: " + path)

	fi, err := os.Stat(path)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	size := fi.Size()
	nChunks := (size + chunkSize - 1) / chunkSize

	nw := runtime.NumCPU()
	if nw > 32 {
		nw = 32
	}
	var next atomic.Int64
	results := make([]*result, nw)
	var wg sync.WaitGroup
	for w := 0; w < nw; w++ {
		results[w] = &result{spill: map[string]int64{}, names: newNameTable()}
		wg.Add(1)
		go func(w int) {
			defer wg.Done()
			f, err := os.Open(path)
			if err != nil {
				return
			}
			defer f.Close()
			buf := make([]byte, bufSize)
			r := results[w]
			for {
				ci := next.Add(1) - 1
				if ci >= nChunks {
					return
				}
				start := ci * chunkSize
				end := start + chunkSize
				if end > size {
					end = size
				}
				processChunk(f, buf, start, end, ci == 0, r)
			}
		}(w)
	}
	wg.Wait()

	var total int64
	months := map[string]int64{}
	nameCount := map[string]int64{}
	var name433, name43244 string
	for _, r := range results {
		total += r.lines
		for idx, c := range r.months {
			if c > 0 {
				y := monthBase + idx/12
				m := idx%12 + 1
				months[fmt.Sprintf("%04d%02d", y, m)] += c
			}
		}
		for k, c := range r.spill {
			months[k] += c
		}
		for i := range r.names.slots {
			s := &r.names.slots[i]
			if s.cnt > 0 {
				nameCount[string(r.names.arena[s.off:s.off+s.nlen])] += s.cnt
			}
		}
		if r.name433 != "" {
			name433 = r.name433
		}
		if r.name4324 != "" {
			name43244 = r.name4324
		}
	}

	if (name433 == "" || name43244 == "") && total >= 43244 {
		// chunk 0 was smaller than 43244 lines; cheap sequential rescan
		f, err := os.Open(path)
		if err == nil {
			rdr := make([]byte, 32<<20)
			n, _ := f.ReadAt(rdr, 0)
			f.Close()
			pos, ln := 0, int64(0)
			for pos < n && ln < 43244 {
				ln++
				e := n
				if j := bytes.IndexByte(rdr[pos:n], '\n'); j >= 0 {
					e = pos + j
				}
				if ln == 433 || ln == 43244 {
					line := rdr[pos:e]
					c, fs := 0, 0
					for i2, b := range line {
						if b == '|' {
							c++
							if c == 7 {
								fs = i2 + 1
							} else if c == 8 {
								if ln == 433 {
									name433 = string(line[fs:i2])
								} else {
									name43244 = string(line[fs:i2])
								}
								break
							}
						}
					}
				}
				pos = e + 1
			}
		}
	}

	var maxName string
	var maxVal int64
	for k, v := range nameCount {
		if v > maxVal {
			maxName, maxVal = k, v
		}
	}

	monthKeys := make([]string, 0, len(months))
	for k := range months {
		monthKeys = append(monthKeys, k)
	}
	sort.Strings(monthKeys)

	fmt.Println("task 1:: ", total)
	fmt.Println("task 2:: ", []string{name433, name43244})
	fmt.Println("task 3:: ")
	for _, k := range monthKeys {
		fmt.Println("  ", k, ":", months[k])
	}
	fmt.Println("task 4:: ", maxName, maxVal)
	fmt.Printf("took:: %dms\n", time.Since(startTime).Milliseconds())
}
