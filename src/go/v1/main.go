package main

import (
    "fmt"
    "os"
    "bufio"
)

func main() {
    inputFile := os.Args[1]  // first is it self

    fmt.Println("processing file:: " + inputFile)

    file, _ := os.Open(inputFile)
    fscanner := bufio.NewScanner(file)
    counter := 0 

    for fscanner.Scan() {
        line := fscanner.Text()
        if len(line) == 0 {
            fmt.Println("line emtpy")
        }
        counter += 1
    }

    fmt.Println("task 1:: ", counter)
    fmt.Println("task 2:: ", counter)
    fmt.Println("task 3:: ", counter)
    fmt.Println("task 4:: ", counter)
}