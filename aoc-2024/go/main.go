package main

import (
	// "aoc/day1"
	"aoc/day2"
	. "aoc/utils"
	"fmt"
)

func main() {
	data, err := ReadFile("./data/day2.txt")
	if err != nil {
		panic(err)
	}

	// fmt.Printf("Answer: %v\n", day1.TotalDistance(data))
	// fmt.Printf("Answer: %v\n", day1.SimilarityScore(data))
	res, err := day2.CountSafeLevels(data)
	if err != nil {
		_ = fmt.Errorf(err.Error())
	}
	fmt.Printf("Answer: %v\n", res)

}
