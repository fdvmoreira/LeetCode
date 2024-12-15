package main

import (
	"aoc/day11"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day11.txt")
	if err != nil {
		panic(err)
	}

	res, err := day11.CountStones(data[0], 75)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
