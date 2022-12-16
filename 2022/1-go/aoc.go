package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
	"strconv"
)

func main() {
	lines := read(os.Stdin)
	first(lines)
	second(lines)
}

func first(lines []string) {
	max := 0
	current := 0
	for _, line := range lines {
		if line == "" {
			if current > max {
				max = current
			}
			current = 0
		}
		lineInt, _ := strconv.Atoi(line)
		current += lineInt
	}
	println(max)
}

func second(lines []string) {
	var elves []int
	current := 0
	for _, line := range lines {
		if line == "" {

			elves = append(elves, current)
			current = 0
		}
		lineInt, _ := strconv.Atoi(line)
		current += lineInt
	}
	fmt.Println(elves)
	sort.Ints(elves)
	res := 0
	for _, elf := range elves[len(elves)-3:] {
		println(elf)
		res += elf
	}
	println(res)
}

func read(source io.Reader) []string {
	reader := bufio.NewScanner(source)
	var lines []string
	for reader.Scan() {
		line := reader.Text()
		lines = append(lines, line)
	}

	return lines
}
