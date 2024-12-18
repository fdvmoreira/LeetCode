package day13

import (
	"errors"
)

func SumSmallestTokenAmount(puzzle *[]string) (int, error) {
	tokensCount := 0

	if len(*puzzle) == 0 {
		return tokensCount, errors.New("No Data")
	}

	return tokensCount, nil
}
