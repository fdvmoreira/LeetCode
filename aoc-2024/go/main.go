package main

import (
	"aoc/day3"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day3.txt")
	if err != nil {
		panic(err)
	}

	res, err := day3.SumOfMultiplicationV2(data)
	if err != nil {
		_ = fmt.Errorf(err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
