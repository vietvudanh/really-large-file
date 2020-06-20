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
}

type LineIndex struct {
	index int64
	line  string
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
	mutex := &sync.Mutex{}
	names := make([]string, 2)
	maxName := ""
	maxVal := 0

	linesChunkLen := 64 * 1024
	lines := make([]LineIndex, 0, 0)

	fsScanner.Scan()
	for {
		counter++
		line := fsScanner.Text()
		li := LineIndex{counter, line}
		// fmt.Println(counter)
		lines = append(lines, li)

		willScan := fsScanner.Scan()

		if len(lines) == linesChunkLen || !willScan {
			process := lines
			wg.Add(len(process))
			go func() {
				entryList := make([]Entry, 0, len(process))
				for _, li := range process {
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
					}
					entryList = append(entryList, e)
				}

				mutex.Lock()
				for _, entry := range entryList {
					if entry.firstName != "" {
						firstNameCount[entry.firstName]++
						if firstNameCount[entry.firstName] > maxVal {
							maxVal = firstNameCount[entry.firstName]
							maxName = entry.firstName
						}
					}
					if entry.index == 433 || entry.index == 43244 {
						names = append(names, entry.name)
					}
					mapDateCount[entry.month]++
				}
				mutex.Unlock()
				wg.Add(-len(process))
			}() // end go func
			lines = make([]LineIndex, 0, linesChunkLen)
		}

		if !willScan {
			break
		}
	}
	wg.Wait()

	fmt.Println("task 1:: ", counter)
	fmt.Println("task 2:: ", names)
	fmt.Println("task 3:: ")
	for k, v := range mapDateCount {
		fmt.Println("  ", k, ":", v)
	}
	fmt.Println("task 4:: ", maxName, maxVal)
}
