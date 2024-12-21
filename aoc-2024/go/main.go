package main

import (
	day14 "aoc/day14"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day14.txt")
	if err != nil {
		panic(err)
	}

	res, err := day14.SafetyFactor(&data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
