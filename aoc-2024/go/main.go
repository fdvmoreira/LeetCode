package main

import (
	"aoc/day6"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day6.txt")
	if err != nil {
		panic(err)
	}

	res, err := day6.VisitedPositions(data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
