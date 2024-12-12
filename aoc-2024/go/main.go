package main

import (
	"aoc/day8"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day8.txt")
	if err != nil {
		panic(err)
	}

	res, err := day8.CountAntinode(&data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
