package day6

import (
	"errors"
	"strings"
)

type Direction int

type Position struct {
	x, y int
}

const (
	UP Direction = iota
	DOWN
	LEFT
	RIGHT
)

func VisitedPositions(mappedArea []string) (int, error) {
	count := 0

	if len(mappedArea) == 0 {
		return 0, errors.New("Empty map")
	}

	// find the guard and its direction
	var guardPos Position

	for row, line := range mappedArea {
		col := strings.Index(line, "^")
		if col != -1 {
			guardPos = Position{col, row}
			break
		}
	}

	// walk in the direction while in the map
	for {

		move(mappedArea, &guardPos)

		// exit if the guard has moved out of mapped area
		if guardPos.x >= len(mappedArea[0]) && guardPos.y >= len(mappedArea) {
			break
		}

		count += 1
	}

	// - increment the steps taken by 1 each time
	return count, nil
}

func move(mappedArea []string, guardPos *Position) {
	// - if an obstacle if in front turn right 90* and keep walking
	// TODO: Get direction
	// TODO: check for obstacle
	// TODO: turn right if obstacle in front
	// TODO: one step in the direction faced by the guard
}
