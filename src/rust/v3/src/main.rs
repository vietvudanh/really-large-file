// v3: share-nothing parallel workers over pread, work-stealing chunks.
//
// v2 bottlenecks: one reader thread doing read_line (one String alloc + copy
// per line), lines cloned into batches, every worker serializing on the same
// five Arc<Mutex<...>>.
//
// Design notes (measured on Apple M2, macOS):
//   - pread per worker, NOT mmap: macOS serializes concurrent page faults on
//     the same mapped file behind a per-VM-object lock, which caps an mmap
//     version of this exact code at ~1 core. read() copies from the page
//     cache in parallel just fine.
//   - work-stealing 64MB chunks instead of one static chunk per core: the M2
//     has 4 performance + 4 efficiency cores, so equal static splits finish
//     whenever the slowest E-core does; stealing keeps all cores busy.
//   - near-zero allocations per line: fields located with memchr (SIMD),
//     months counted in a flat per-worker array indexed by (year*12+month),
//     names counted in a per-worker FxHashMap (owned keys allocate only on
//     first sighting of a name). Merge happens once at the end.

use memchr::memchr;
use rustc_hash::FxHashMap;
use std::env;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

const MONTH_SLOTS: usize = 1024;
const MONTH_BASE: i32 = 1990; // idx = (year-MONTH_BASE)*12 + month-1
const CHUNK_SIZE: u64 = 128 << 20;
const BUF_SIZE: usize = 16 << 20;

struct Res {
    lines: u64,
    months: Vec<u64>,
    spill: FxHashMap<Vec<u8>, u64>,
    names: FxHashMap<Vec<u8>, u64>,
    cap433: Option<Vec<u8>>,
    cap43244: Option<Vec<u8>>,
}

impl Res {
    fn new() -> Self {
        Res {
            lines: 0,
            months: vec![0u64; MONTH_SLOTS],
            spill: FxHashMap::default(),
            names: FxHashMap::with_capacity_and_hasher(1 << 17, Default::default()),
            cap433: None,
            cap43244: None,
        }
    }

    fn bump(map: &mut FxHashMap<Vec<u8>, u64>, key: &[u8]) {
        if let Some(v) = map.get_mut(key) {
            *v += 1;
        } else {
            map.insert(key.to_vec(), 1);
        }
    }

    fn process_line(&mut self, line: &[u8], capture: bool) {
        self.lines += 1;
        let mut p = 0usize;
        for _ in 0..4 {
            // skip fields 0-3
            match memchr(b'|', &line[p..]) {
                Some(j) => p += j + 1,
                None => return,
            }
        }
        let ds = p;
        let de = match memchr(b'|', &line[p..]) {
            Some(j) => p + j,
            None => return,
        };
        p = de + 1;
        for _ in 0..2 {
            // skip fields 5-6
            match memchr(b'|', &line[p..]) {
                Some(j) => p += j + 1,
                None => return,
            }
        }
        let ns = p;
        let ne = match memchr(b'|', &line[p..]) {
            Some(j) => p + j,
            None => return,
        };

        let date = &line[ds..de];
        let name = &line[ns..ne];

        if date.len() >= 6 && date[..6].iter().all(|c| c.is_ascii_digit()) {
            let y = (date[0] - b'0') as i32 * 1000
                + (date[1] - b'0') as i32 * 100
                + (date[2] - b'0') as i32 * 10
                + (date[3] - b'0') as i32;
            let m = (date[4] - b'0') as i32 * 10 + (date[5] - b'0') as i32;
            let idx = (y - MONTH_BASE) * 12 + m - 1;
            if (0..MONTH_SLOTS as i32).contains(&idx) {
                self.months[idx as usize] += 1;
            } else {
                Self::bump(&mut self.spill, &date[..6]);
            }
        } else if date.len() >= 6 {
            Self::bump(&mut self.spill, &date[..6]);
        } else {
            Self::bump(&mut self.spill, date);
        }

        // same rule as v1/v2: gate on ", " anywhere, key = before FIRST
        // comma, skip empty keys
        if let Some(ci) = memchr(b',', name) {
            if ci > 0 {
                let matched = name.get(ci + 1) == Some(&b' ')
                    || (ci + 1 < name.len()
                        && name[ci + 1..].windows(2).any(|w| w == b", "));
                if matched {
                    Self::bump(&mut self.names, &name[..ci]);
                }
            }
        }

        if capture {
            if self.lines == 433 {
                self.cap433 = Some(name.to_vec());
            } else if self.lines == 43244 {
                self.cap43244 = Some(name.to_vec());
            }
        }
    }
}

// Handles every line that STARTS in [start,end); the final line may extend
// past end and is read to completion. When start > 0 the bytes up to the
// first newline belong to the previous chunk and are skipped. capture is set
// only for chunk 0, whose local line numbers are global.
fn process_chunk(f: &File, buf: &mut [u8], start: u64, end: u64, capture: bool, r: &mut Res) {
    let mut buf_off = start;
    let mut carry = 0usize;
    let mut aligned = start == 0;

    loop {
        let n = f.read_at(&mut buf[carry..], buf_off + carry as u64).unwrap_or(0);
        let valid = carry + n;
        let eof = n == 0;
        if valid == 0 {
            return;
        }

        let mut p = 0usize;
        if !aligned {
            match memchr(b'\n', &buf[..valid]) {
                Some(j) => {
                    p = j + 1;
                    aligned = true;
                    if buf_off + p as u64 >= end {
                        return;
                    }
                }
                None => {
                    if eof {
                        return;
                    }
                    buf_off += valid as u64;
                    carry = 0;
                    continue;
                }
            }
        }

        while p < valid {
            if buf_off + p as u64 >= end {
                return;
            }
            match memchr(b'\n', &buf[p..valid]) {
                Some(nl) => {
                    r.process_line(&buf[p..p + nl], capture);
                    p += nl + 1;
                }
                None => {
                    if eof {
                        // final line without trailing newline
                        r.process_line(&buf[p..valid], capture);
                        return;
                    }
                    break;
                }
            }
        }
        if eof && p >= valid {
            return;
        }
        carry = valid - p;
        if carry == valid && valid == buf.len() {
            return; // single line larger than buffer; not this data
        }
        buf.copy_within(p..valid, 0);
        buf_off += p as u64;
    }
}

fn main() {
    let start_time = Instant::now();
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: v3 <file>");
            process::exit(1);
        }
    };
    println!("processing file:: {}", path);

    let size = std::fs::metadata(&path).expect("cannot stat file").len();
    let n_chunks = (size + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let next = AtomicU64::new(0);

    let n = thread::available_parallelism()
        .map(|v| v.get())
        .unwrap_or(8)
        .min(32);

    let results: Vec<Res> = thread::scope(|s| {
        let handles: Vec<_> = (0..n)
            .map(|_| {
                let path = &path;
                let next = &next;
                s.spawn(move || {
                    let mut r = Res::new();
                    let f = match File::open(path) {
                        Ok(f) => f,
                        Err(_) => return r,
                    };
                    let mut buf = vec![0u8; BUF_SIZE];
                    loop {
                        let ci = next.fetch_add(1, Ordering::Relaxed);
                        if ci >= n_chunks {
                            return r;
                        }
                        let start = ci * CHUNK_SIZE;
                        let end = (start + CHUNK_SIZE).min(size);
                        process_chunk(&f, &mut buf, start, end, ci == 0, &mut r);
                    }
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    let mut total: u64 = 0;
    let mut months: FxHashMap<String, u64> = FxHashMap::default();
    let mut names: FxHashMap<Vec<u8>, u64> = FxHashMap::default();
    let mut cap433: Option<Vec<u8>> = None;
    let mut cap43244: Option<Vec<u8>> = None;
    for r in results {
        total += r.lines;
        for (idx, c) in r.months.iter().enumerate() {
            if *c > 0 {
                let y = MONTH_BASE + (idx as i32) / 12;
                let m = (idx as i32) % 12 + 1;
                *months.entry(format!("{:04}{:02}", y, m)).or_insert(0) += c;
            }
        }
        for (k, c) in r.spill {
            *months
                .entry(String::from_utf8_lossy(&k).into_owned())
                .or_insert(0) += c;
        }
        for (k, c) in r.names {
            *names.entry(k).or_insert(0) += c;
        }
        if r.cap433.is_some() {
            cap433 = r.cap433;
        }
        if r.cap43244.is_some() {
            cap43244 = r.cap43244;
        }
    }

    if (cap433.is_none() || cap43244.is_none()) && total >= 43244 {
        // chunk 0 was smaller than 43244 lines; cheap sequential rescan
        if let Ok(f) = File::open(&path) {
            let mut rdr = vec![0u8; 32 << 20];
            let n = f.read_at(&mut rdr, 0).unwrap_or(0);
            let (mut pos, mut ln) = (0usize, 0u64);
            while pos < n && ln < 43244 {
                ln += 1;
                let e = match memchr(b'\n', &rdr[pos..n]) {
                    Some(j) => pos + j,
                    None => n,
                };
                if ln == 433 || ln == 43244 {
                    let line = &rdr[pos..e];
                    let (mut c, mut fs) = (0, 0usize);
                    for (i, b) in line.iter().enumerate() {
                        if *b == b'|' {
                            c += 1;
                            if c == 7 {
                                fs = i + 1;
                            } else if c == 8 {
                                let nm = line[fs..i].to_vec();
                                if ln == 433 {
                                    cap433 = Some(nm);
                                } else {
                                    cap43244 = Some(nm);
                                }
                                break;
                            }
                        }
                    }
                }
                pos = e + 1;
            }
        }
    }

    let mut max_name: &[u8] = b"";
    let mut max_val: u64 = 0;
    for (k, v) in &names {
        if *v > max_val {
            max_name = k;
            max_val = *v;
        }
    }

    let mut month_keys: Vec<&String> = months.keys().collect();
    month_keys.sort();

    let fmt_name = |o: &Option<Vec<u8>>| -> String {
        o.as_ref()
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .unwrap_or_default()
    };

    println!("task 1:: {}", total);
    println!(
        "task 2:: [\"{}\", \"{}\"]",
        fmt_name(&cap433),
        fmt_name(&cap43244)
    );
    println!("task 3:: ");
    for k in month_keys {
        println!("   {} : {}", k, months[k]);
    }
    println!(
        "task 4:: {}, {}",
        String::from_utf8_lossy(max_name),
        max_val
    );
    println!("took:: {}ms", start_time.elapsed().as_millis());
}
