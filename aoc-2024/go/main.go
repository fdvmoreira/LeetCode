package main

import (
	"aoc/day10"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day10.txt")
	if err != nil {
		panic(err)
	}

	res, err := day10.SumTrailheadsUniquePathScore(&data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
