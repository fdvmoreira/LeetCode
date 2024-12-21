package day14

import (
	"errors"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type Location struct {
	x, y int
}

type Robot struct {
	position, velocity Location
}

func SafetyFactor(data *[]string) (int, error) {
	if len(*data) == 0 {
		return 0, errors.New("No Data!")
	}

	factor := 0

	var robots = make([]Robot, len(*data))

	// create the robots
	for idx, line := range *data {
		var tmp = strings.Fields(line)

		var extractedNums [4]int
		position, velocity := tmp[0], tmp[1]

		positionXY := strings.Split(strings.Split(position, "=")[1], ",")
		velocityXY := strings.Split(strings.Split(velocity, "=")[1], ",")

		for i, v := range slices.Concat(positionXY, velocityXY) {
			num, err := strconv.Atoi(v)
			if err != nil {
				return 0, fmt.Errorf("Something went wrong!\n%v\n", err.Error())
			}
			extractedNums[i] = num
		}

		newRobot := Robot{}

		newRobot.position.x = extractedNums[0]
		newRobot.position.y = extractedNums[1]
		newRobot.velocity.x = extractedNums[2]
		newRobot.velocity.y = extractedNums[3]

		robots[idx] = newRobot
	}

	const (
		tileWidth  = 101 //11 // NOTE: change for testing
		tileHeight = 103 //7  // NOTE: change for testing
	)

	// simulate the postion of robots after 100s
	for i := 0; i < 100; i++ {
		for j, robot := range robots {

			var (
				p = robot.position
				v = robot.velocity
			)
			robots[j].position.x = (p.x + (v.x)%tileWidth + tileWidth) % tileWidth
			robots[j].position.y = (p.y + (v.y)%tileHeight + tileHeight) % tileHeight
		}
	}

	var quadrants = []int{0, 0, 0, 0}
	const (
		q1 int = iota
		q2
		q3
		q4
	)

	for _, robot := range robots {
		if robot.position.x < tileWidth/2 && robot.position.y < tileHeight/2 {
			quadrants[q1] += 1
		} //q1

		if robot.position.x > tileWidth/2 && robot.position.y < tileHeight/2 {
			quadrants[q2] += 1
		} //q2

		if robot.position.x < tileWidth/2 && robot.position.y > tileHeight/2 {
			quadrants[q3] += 1
		} //q3

		if robot.position.x > tileWidth/2 && robot.position.y > tileHeight/2 {
			quadrants[q4] += 1
		} //q4
	}

	factor = func(qd []int) (res int) {
		res = qd[q1] * qd[q2] * qd[q3] * qd[q4]
		return
	}(quadrants)

	return factor, nil
}
