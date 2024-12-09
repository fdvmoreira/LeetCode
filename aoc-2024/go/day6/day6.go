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
	if len(mappedArea) == 0 {
		return 0, errors.New("Empty map")
	}

	count := 1

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
		dir := move(&mappedArea, &guardPos)
		// exit if the guard has moved out of mapped area
		if (dir == RIGHT && (guardPos.x == len(mappedArea[0])-1)) || // left mapped area >
			(dir == DOWN && (guardPos.y == len(mappedArea)-1)) || // left mapped area v
			(dir == UP && guardPos.y == 0) || // left mapped area ^
			(dir == LEFT && guardPos.x == 0) { // left mapped area <
			break
		}
	}

	// count the distinct visited positions
	for _, line := range mappedArea {
		count += strings.Count(line, "X")
	}

	return count, nil
}

func move(mappedArea *[]string, guardPos *Position) Direction {
	//WARN: The positions might because negative if move up of left and that panics! So ensure that give position are correct according to the direction
	//
	var directions = map[string]Direction{
		"^": UP,
		">": RIGHT,
		"v": DOWN,
		"<": LEFT,
	}

	var currentDir Direction
	// TODO: Get direction
	guard := (*mappedArea)[guardPos.y][guardPos.x]
	currentDir = directions[string(guard)]

	// TODO: check for obstacle
	currentRow := strings.Split((*mappedArea)[guardPos.y], "")
	switch currentDir {
	case UP:
		if (*mappedArea)[guardPos.y-1][guardPos.x] == '#' {
			currentRow[guardPos.x] = ">"
			(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
			currentDir = RIGHT
			break
		}
		currentRow[guardPos.x] = "X"
		(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
		nextRow := strings.Split((*mappedArea)[guardPos.y-1], "")
		nextRow[guardPos.x] = "^"
		(*mappedArea)[guardPos.y-1] = strings.Join(nextRow, "")
		(*guardPos).y -= 1

	case RIGHT:
		if (*mappedArea)[guardPos.y][guardPos.x+1] == '#' {
			currentRow[guardPos.x] = "v"
			(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
			currentDir = DOWN
			break
		}
		currentRow[guardPos.x] = "X"
		currentRow[guardPos.x+1] = ">"
		(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
		(*guardPos).x += 1

	case DOWN:
		if (*mappedArea)[guardPos.y+1][guardPos.x] == '#' {
			currentRow[guardPos.x] = "<"
			(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
			currentDir = LEFT
			break
		}
		currentRow[guardPos.x] = "X"
		(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
		nextRow := strings.Split((*mappedArea)[guardPos.y+1], "")
		nextRow[guardPos.x] = "v"
		(*mappedArea)[guardPos.y+1] = strings.Join(nextRow, "")
		(*guardPos).y += 1

	case LEFT:
		if (*mappedArea)[guardPos.y][guardPos.x-1] == '#' {
			currentRow[guardPos.x] = "^"
			(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
			currentDir = UP
			break
		}
		currentRow[guardPos.x] = "X"
		currentRow[guardPos.x-1] = "<"
		(*mappedArea)[guardPos.y] = strings.Join(currentRow, "")
		(*guardPos).x -= 1

	default:
		panic("Something went wrong")
	}

	return currentDir
}
