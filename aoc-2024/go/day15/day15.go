package day15

import (
	"errors"
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

	for rIdx, row := range warehouse {
		for cIdx, x := range row {
			if x == 'O' {
				sumBoxesGPSCoords += 100*rIdx + cIdx
			}
		}
	}

	return sumBoxesGPSCoords, nil
}

func move(r *Robot, dir Direction, w *[][]byte) (bool, error) {
	if r.position.x == 0 || r.position.y == 0 {
		return false, errors.New("Robot Not Found")
	}

	currPos := (*r).position
	canMove := true

	for (*w)[currPos.y][currPos.x] != '.' {
		if dir == Up {
			currPos.y -= 1
		}

		if dir == Right {
			currPos.x += 1
		}

		if dir == Down {
			currPos.y += 1
		}

		if dir == Left {
			currPos.x -= 1
		}

		if (*w)[currPos.y][currPos.x] == '#' {
			canMove = false
			currPos = (*r).position
			break
		}
	}

	if canMove {
		initialRobotPos := (*r).position

		// shift the object in the direction of the found empty slot
		currentObj := &(*w)[currPos.y][currPos.x]

		if dir == Up {

			newRobotPos := &(*w)[initialRobotPos.y-1][initialRobotPos.x]
			*currentObj = *newRobotPos
			*newRobotPos = (*w)[initialRobotPos.y][initialRobotPos.x]
			(*w)[initialRobotPos.y][initialRobotPos.x] = '.'
			(*r).position.y -= 1

		}

		if dir == Right {

			newRobotPos := &(*w)[initialRobotPos.y][initialRobotPos.x+1]
			*currentObj = *newRobotPos
			*newRobotPos = (*w)[initialRobotPos.y][initialRobotPos.x]
			(*w)[initialRobotPos.y][initialRobotPos.x] = '.'
			(*r).position.x += 1
		}

		if dir == Down {

			newRobotPos := &(*w)[initialRobotPos.y+1][initialRobotPos.x]
			*currentObj = *newRobotPos
			*newRobotPos = (*w)[initialRobotPos.y][initialRobotPos.x]
			(*w)[initialRobotPos.y][initialRobotPos.x] = '.'
			(*r).position.y += 1

		}

		if dir == Left {

			newRobotPos := &(*w)[initialRobotPos.y][initialRobotPos.x-1]
			*currentObj = *newRobotPos
			*newRobotPos = (*w)[initialRobotPos.y][initialRobotPos.x]
			(*w)[initialRobotPos.y][initialRobotPos.x] = '.'
			(*r).position.x -= 1

		}
	}

	return canMove, nil
}
