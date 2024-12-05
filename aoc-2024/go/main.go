package main

import (
	"aoc/day4"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day4.txt")
	if err != nil {
		panic(err)
	}

	res, err := day4.XMasWordSearch(data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
