package main

import "fmt"

func main() {
	y := []int{1, 2, 3}
	// var test = y[1:3]
	y[2] = 99
	z := []int{}
	fmt.Println(len(z))

	incAll(y)
	fmt.Println(y)

	a := make([]int, 1)
	fmt.Println(a, cap(a))

	a = append(a, 100)
	fmt.Println(a, cap(a))

	a = append(a, 100)
	fmt.Println(a, cap(a))

	a = append(a, 100)
	fmt.Println(a, cap(a))

	a = append(a, 100)
	fmt.Println(a, cap(a))
}

func incAll(array []int) {
	for i := range array {
		array[i]++
	}
	fmt.Println(array)

}
