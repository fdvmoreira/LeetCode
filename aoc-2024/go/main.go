package main

import (
	"aoc/day9"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day9.txt")
	if err != nil {
		panic(err)
	}

	res, err := day9.FSChecksum(&data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
