package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func main() {
	var scan = bufio.NewScanner(os.Stdin)

	var leftList []int
	var rightList []int

	for scan.Scan() {
		line := scan.Text()
		words := strings.Fields(line)
		left, _ := strconv.Atoi(words[0])
		right, _ := strconv.Atoi(words[1])
		leftList = append(leftList, left)
		rightList = append(rightList, right)
	}
	dups := make(map[int]int)
	output := 0
	for _, value := range rightList {
		dups[value]++
	}
	for _, value := range leftList {
		output += value * dups[value]
	}
	fmt.Println(output)
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

func sum(list []int) int {
	total := 0
	for _, value := range list {
		total += value
	}

	return total
}
