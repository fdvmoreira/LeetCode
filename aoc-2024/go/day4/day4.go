package day4

import (
	"errors"
	"strings"
)

func XMasWordSearch(data []string) (int, error) {
	count := 0

	if len(data) == 0 {
		return count, errors.New("Data Empty!")
	}

	for row, xmas := range data {
		for col, ch := range xmas {
			if ch == 'X' {
				// Rotate around
				count += LookAround(row, col, &data)
			}
		}
	}

	return count, nil
}

func LookAround(row, col int, data *[]string) (matches int) {
	var (
		// topBound    = 0
		rightBound  = len((*data)[row]) - 1
		bottomBound = len(*data) - 1
		// leftBound   = 0
		search = "MAS"
	)
	// ->
	if col+3 <= rightBound && strings.Compare((*data)[row][col:col+3], search) == 0 {
		matches += 1
	}
	// \v
	if col+3 <= rightBound && row+3 <= bottomBound {
		txt := [...]string{string((*data)[row+1][col+1]), string((*data)[row+2][col+2]), string((*data)[row+3][col+3])}
		if strings.Compare(strings.Join(txt[:], ""), search) == 0 {
			matches += 1
		}
	}

	// v
	// TODO:Look Down

	// /

	// <-

	// \^

	// ^

	// /^

	return
}
