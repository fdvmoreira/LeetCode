package day15

import (
	"errors"
	"fmt"
	"strings"
)

type Coord struct {
	x, y int
}

type Robot struct {
	position Coord
}

type Box struct {
	location Coord
}

type Direction byte

const (
	Up    Direction = '^'
	Down  Direction = 'v'
	Right Direction = '>'
	Left  Direction = '<'
)

func SumBoxesGPSCoordinates(input *[]string) (int, error) {
	if len(*input) == 0 {
		return 0, errors.New("No Data")
	}
	sumBoxesGPSCoords := 0

	var (
		warehouse   [][]byte
		movement    []byte
		wmSeparator int
		robot       Robot
	)

	for i, line := range *input {
		if len(line) == 0 {
			wmSeparator = i
			break
		}
		warehouse = append(warehouse, []byte(line))
	}

	movement = []byte(strings.Join((*input)[wmSeparator:], ""))

	for i, v := range *input {
		if rIdx := strings.IndexByte(v, '@'); rIdx != -1 {
			robot = Robot{
				Coord{
					rIdx,
					i,
				},
			}
		}
	}

	for _, v := range movement {
		_, err := move(&robot, Direction(v), &warehouse)
		if err != nil {
			return 0, err
		}
	}

	return sumBoxesGPSCoords, nil
}

func move(r *Robot, dir Direction, w *[][]byte) (bool, error) {
	moved := false
	if r.position.x == 0 || r.position.y == 0 {
		return false, errors.New("Robot Not Found")
	}

	// TODO: Find the empty slot
	// Move the robot and every box to the found slot
	canMove, lastObjToMove := func(r *Robot, dir Direction, w *[][]byte) (bool, Coord) {
		currPos := r.position
		for {
			if (*w)[currPos.y][currPos.x] == ' ' {
				return true, currPos
			}
		}
		return false, Coord{}
	}(r, dir, w)

	switch dir {
	case Up:

	case Right:

	case Down:

	case Left:

	default:
		return false, fmt.Errorf("Unknown Movement: %v", string(dir))
	}

	return moved, nil
}

func canMove(r *Robot, dir Direction)
