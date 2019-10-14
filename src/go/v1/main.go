package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	inputFile := os.Args[1] // first is it self

	fmt.Println("processing file:: " + inputFile)

	file, _ := os.Open(inputFile)
	fscanner := bufio.NewScanner(file)
	counter := 0

	// store data
	firstNameCount := make(map[string]int)
	mapDateCount := make(map[string]int)

	names := make([]string, 2)

	for fscanner.Scan() {
		counter++
		line := fscanner.Text()
		if len(line) == 0 {
			fmt.Println("line emtpy")
		}

		splits := strings.SplitN(line, "|", 9)
		date := splits[4]
		name := splits[7]

		month := date[:6]
		mapDateCount[month]++

		if counter == 433 || counter == 43244 {
			names = append(names, name)
		}

		if strings.Contains(name, ", ") {
			splitsName := strings.SplitN(name, ",", 2)
			firstNameCount[splitsName[0]]++
		}
	}

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
