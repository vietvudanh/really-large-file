package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"sync"
)

type Entry struct {
	index     int64
	firstName string
	name      string
	month     string
	wg        *sync.WaitGroup
}

type LineIndex struct {
	index int64
	line  string
}

func processLines(lines []LineIndex, wg *sync.WaitGroup, entries chan<- []Entry) {
	entryList := make([]Entry, 0, len(lines))
	for _, li := range lines {
		line := li.line
		splits := strings.SplitN(line, "|", 9)
		date := splits[4]
		name := splits[7]
		firstName := ""

		month := date[:6]

		if strings.Contains(name, ", ") {
			splitsName := strings.SplitN(name, ",", 2)
			firstName = splitsName[0]
		}

		e := Entry{
			li.index,
			firstName,
			name,
			month,
			wg,
		}

		entryList = append(entryList, e)

	}

	entries <- entryList
}

func main() {
	// trace.Start(os.Stderr)
	// defer trace.Stop()
	inputFile := os.Args[1] // first is it self
	fmt.Println("processing file:: " + inputFile)

	file, _ := os.Open(inputFile)
	fsScanner := bufio.NewScanner(file)
	var counter int64 = 0

	// store data
	firstNameCount := make(map[string]int)
	mapDateCount := make(map[string]int)

	// concurrent
	wg := sync.WaitGroup{}
	entries := make(chan []Entry)
	names := make([]string, 2)

	go func() {
		for {
			select {
			case entryList, ok := <-entries:
				if ok {
					for _, entry := range entryList {
						if entry.firstName != "" {
							firstNameCount[entry.firstName]++
						}
						if entry.index == 433 || entry.index == 43244 {
							names = append(names, entry.name)
						}
						mapDateCount[entry.month]++
						entry.wg.Done()
					}
				}
			}
		}
	}()

	linesChunkLen := 64 * 1024
	lines := make([]LineIndex, 0, 0)
	for fsScanner.Scan() {
		counter++
		line := fsScanner.Text()
		lines = append(lines, LineIndex{counter, line})

		if len(lines) == linesChunkLen {
			wg.Add(len(lines))
			process := lines

			go processLines(process, &wg, entries)

			lines = make([]LineIndex, 0, linesChunkLen)
		}
	}
	if len(lines) > 0 {
		fmt.Println("there are date left::", len(lines))
		wg.Add(len(lines))
		process := lines
		go processLines(process, &wg, entries)
	}

	wg.Wait()
	close(entries)

	maxVal, maxName := 0, ""
	for k, v := range firstNameCount {
		if v > maxVal {
			maxName, maxVal = k, v
		}
	}

	fmt.Println("task 1:: ", counter)
	fmt.Println("task 2:: ", names)
	fmt.Println("task 3:: ")
	for k, v := range mapDateCount {
		fmt.Println("  ", k, ":", v)
	}
	fmt.Println("task 4:: ", maxName, maxVal)
}
