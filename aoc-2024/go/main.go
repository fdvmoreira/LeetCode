package main

import (
	"aoc/day1"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day1-1.txt")
	if err != nil {
		panic(err)
	}

	fmt.Printf("Answer: %v\n", day1.TotalDistance(data))
}
