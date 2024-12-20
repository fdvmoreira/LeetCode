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
					cost: 0,
				}

				clowMachines[selectedClawMachine] = cm
			}

			if strings.HasSuffix(tmpText[0], "B") {
				cm := clowMachines[selectedClawMachine]
				cm.buttonB = Button{
					id:   'B',
					x:    x,
					y:    y,
					cost: 0,
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
		// memo := make(map[Prize]int)
		tokens := SmallestTokenAmountV2(clawMachine)

		tokensCount += tokens
	}

	return tokensCount, nil
}

func SmallestTokenAmountV2(clawMachine ClawMachine) int {
	minCost := math.MaxInt

	for i := 0; i < 100; i++ {
		bAX := i * clawMachine.buttonA.x
		bAY := i * clawMachine.buttonA.y
		for j := 0; j < 100; j++ {
			bBX := j * clawMachine.buttonB.x
			bBY := j * clawMachine.buttonB.y

			if bAX+bBX == clawMachine.prize.x && bAY+bBY == clawMachine.prize.y {
				minCost = min(((i)*3 + (j)*1), minCost)
			}
		}
	}

	if minCost == math.MaxInt {
		return 0
	}

	return minCost
}

// DEPRECATED
func SmallestTokenAmount(clawMachine ClawMachine, memo *map[Prize]int) int {

	if _, ok := (*memo)[clawMachine.prize]; ok {
		return (*memo)[clawMachine.prize]
	}

	if clawMachine.prize.x == 0 && clawMachine.prize.y == 0 {
		return 0 // TODO: found
	}

	if clawMachine.prize.x < 0 || clawMachine.prize.y < 0 {
		return math.MaxInt
	}
	//A
	cma := clawMachine
	cma.prize.x -= cma.buttonA.x
	cma.prize.y -= cma.buttonA.y
	cma.buttonA.cost += 3
	costa := SmallestTokenAmount(cma, memo)
	if costa == 0 {
		costa += cma.buttonA.cost
	}

	//B
	cmb := clawMachine
	cmb.prize.x -= cmb.buttonB.x
	cmb.prize.y -= cmb.buttonB.y
	cmb.buttonB.cost += 2
	costb := SmallestTokenAmount(cmb, memo)
	if costb == 0 {
		costb += cmb.buttonB.cost
	}

	smallest := min(costa, costb)

	(*memo)[clawMachine.prize] = smallest

	return smallest
}
