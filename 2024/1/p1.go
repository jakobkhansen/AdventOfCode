package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
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
	sort.Sort(sort.IntSlice(leftList))
	sort.Sort(sort.IntSlice(rightList))

	var diff = 0

	for i := 0; i < len(leftList); i++ {
		diff += int((float64(leftList[i] - rightList[i])))
	}
	fmt.Println(diff)
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
