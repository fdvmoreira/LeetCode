package main

import (
	"aoc/day7"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day7.txt")
	if err != nil {
		panic(err)
	}

	res, err := day7.CalculateCalibration(data)
	if err != nil {
		fmt.Printf("Error: %v", err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
