package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	inputFile := os.Args[1] // first is it self

	fmt.Println("processing file:: " + inputFile)

	file, _ := os.Open(inputFile)
	fscanner := bufio.NewScanner(file)
	counter := 0

	// store data
	for fscanner.Scan() {
		counter++

	}
	fmt.Println("task 1:: ", counter)
}
