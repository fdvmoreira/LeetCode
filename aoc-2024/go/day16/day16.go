package day16

import "errors"

func MazeCost(maze *[]string) (int, error) {
	totalCost := 0

	if len(*maze) == 0 {
		return totalCost, errors.New("No Data!")
	}

	return totalCost, nil
}
