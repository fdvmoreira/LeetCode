package main

import (
	"aoc/day13"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day13.txt")
	if err != nil {
		panic(err)
	}

	res, err := day13.SumSmallestTokenAmount(&data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
