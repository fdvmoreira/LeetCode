package main

import (
	"aoc/day5"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day5.txt")
	if err != nil {
		panic(err)
	}

	res, err := day5.SumMiddlePages(data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
