package day8

import (
	"errors"
	"strings"
)

type Position struct {
	x, y int
}

func CountAntinode(mapa *[]string) (int, error) {
	const alphaDigit = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
	count := 0
	mapaLen := len(*mapa)
	if mapaLen == 0 {
		return count, errors.New("Empty Data")
	}
	mapaXLen := len((*mapa)[0])

	antenasPos := make(map[string][]Position)
	for row, str := range *mapa {
		for col, r := range str {

			// Assign new position to the exist group or create new group
			if strings.Contains(alphaDigit, string(r)) {
				var positions []Position
				if existingPositions, ok := antenasPos[string(r)]; ok {
					positions = existingPositions
				}
				positions = append(positions, Position{col, row})
				antenasPos[string(r)] = positions
			}
		}
	}

	var antinodes = make(map[Position]bool)

	// find the antinodes
	for _, antenas := range antenasPos {
		for i := 0; i < len(antenas); i++ {
			for j := 0; j < len(antenas); j++ {
				if i == j {
					continue
				}
				nodeLocs := func(a, b Position) []Position {
					var aNodes []Position

					rowDiff := a.y - b.y
					colDiff := a.x - b.x

					var (
						ax, ay, bx, by int
					)

					//Turning negative number into positive
					if rowDiff < 0 {
						rowDiff = ^rowDiff + 1
					}

					if colDiff < 0 {
						colDiff = ^colDiff + 1
					}

					//A
					if a.x > b.x {
						ax = a.x + colDiff
					} else {
						ax = a.x - colDiff
					}

					if a.y < b.y {
						ay = a.y - rowDiff
					} else {
						ay = a.y + rowDiff
					}
					//B
					if b.x > a.x {
						bx = b.x + colDiff

					} else {
						bx = b.x - colDiff
					}

					if b.y > a.y {
						by = b.y + rowDiff
					} else {
						by = b.y - rowDiff
					}

					aNodes = append(aNodes, Position{ax, ay})
					aNodes = append(aNodes, Position{bx, by})

					return aNodes
				}(antenas[i], antenas[j])

				first, second := nodeLocs[0], nodeLocs[1]

				if first.x >= 0 && first.x < mapaXLen && first.y >= 0 && first.y < mapaLen {
					antinodes[first] = true
				}

				if second.x >= 0 && second.x < mapaXLen && second.y >= 0 && second.y < mapaLen {
					antinodes[second] = true
				}

			}
		}
	}

	count = len(antinodes)

	return count, nil
}
