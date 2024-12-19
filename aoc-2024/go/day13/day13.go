package day13

import (
	"errors"
	"fmt"
	"math"
	"strconv"
	"strings"
	"text/scanner"
)

type Prize struct {
	x, y int
}

type Button struct {
	id   byte
	x, y int
	cost int
}

type ClawMachine struct {
	buttonA Button
	buttonB Button
	prize   Prize
}

func SumSmallestTokenAmount(puzzle *[]string) (int, error) {
	tokensCount := 0

	if len(*puzzle) == 0 {
		return tokensCount, errors.New("No Data")
	}

	var (
		clowMachines = []ClawMachine{
			ClawMachine{},
		}
		selectedClawMachine = 0
	)

	// create claw machine
	for _, v := range *puzzle {

		if len(v) == 0 {
			clowMachines = append(clowMachines, ClawMachine{})
			selectedClawMachine += 1
		}

		if len(v) != 0 {
			tmpText := strings.Split(v, ":")

			const (
				xIndex = 2
				yIndex = 6
			)
			var arr [7]string

			var s scanner.Scanner
			s.Init(strings.NewReader(tmpText[1]))
			for tok, ii := s.Scan(), 0; tok != scanner.EOF; ii, tok = ii+1, s.Scan() {
				arr[ii] = s.TokenText()
			}

			x, err := strconv.Atoi(arr[xIndex])
			if err != nil {
				return 0, fmt.Errorf("Conversion Error: %s", err.Error())
			}
			y, err := strconv.Atoi(arr[yIndex])
			if err != nil {
				return 0, fmt.Errorf("Conversion Error: %s", err.Error())
			}

			if strings.HasSuffix(tmpText[0], "A") {
				cm := clowMachines[selectedClawMachine]
				cm.buttonA = Button{
					id:   'A',
					x:    x,
					y:    y,
					cost: 3,
				}

				clowMachines[selectedClawMachine] = cm
			}

			if strings.HasSuffix(tmpText[0], "B") {
				cm := clowMachines[selectedClawMachine]
				cm.buttonB = Button{
					id:   'B',
					x:    x,
					y:    y,
					cost: 1,
				}

				clowMachines[selectedClawMachine] = cm
			}

			if strings.HasSuffix(tmpText[0], "rize") {
				cm := clowMachines[selectedClawMachine]
				cm.prize = Prize{
					x: x,
					y: y,
				}
				clowMachines[selectedClawMachine] = cm
			}
		}
	}

	for _, clawMachine := range clowMachines {
		tokens := SmallestTokenAmount(clawMachine)

		tokensCount += tokens
	}

	return tokensCount, nil
}

func SmallestTokenAmount(clawMachine ClawMachine) int {

	if clawMachine.prize.x == 0 && clawMachine.prize.y == 0 {
		return 0 // TODO: found
	}

	return 0
}
