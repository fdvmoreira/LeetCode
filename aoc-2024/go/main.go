package main

import (
	day15 "aoc/day15"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day15.txt")
	if err != nil {
		panic(err)
	}

	res, err := day15.SumBoxesGPSCoordinates(&data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
