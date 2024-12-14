package day10

import (
	"container/list"
	"errors"
)

func SumTrailheadsScore(topographicMap *[]string) (int, error) {
	scoresSum := 0
	if len(*topographicMap) == 0 {
		return scoresSum, errors.New("No Data")
	}

	type Location struct {
		x, y int
	}

	var trailheads = make(map[Location]int)

	// find possible trail head trailheads
	for i, v := range *topographicMap {
		for j, r := range v {
			if r == '0' {
				trailheads[Location{j, i}] = 0
			}
		}
	}

	// - find hiking trails
	func(theads *map[Location]int, tmap *[]string) {
		//
		for key := range *theads {
			stack := list.New()
			stack.PushBack(key) // Starting Position
			paths := make(map[Location]bool)

			for stack.Len() != 0 {
				// check if the current top of stack is the target we looking for
				el := stack.Back()
				stack.Remove(el)
				pos := el.Value.(Location)
				row, col := pos.y, pos.x
				currVal := (*tmap)[row][col]

				if currVal == '9' {
					paths[Location{col, row}] = true //memoize found value locations
					stack.Remove(el)
					continue
				}

				// If there are still elemet after previous removal

				// look around
				// left
				if col > 0 {
					lVal := (*tmap)[row][col-1]
					if lVal == (currVal + 1) {
						stack.PushBack(Location{col - 1, row})
					}
				}
				// up
				if row > 0 {
					tVal := (*tmap)[row-1][col]
					if tVal == (currVal + 1) {
						stack.PushBack(Location{col, row - 1})
					}
				}
				// right
				if col < len((*tmap)[0])-1 {
					rVal := (*tmap)[row][col+1]
					if rVal == (currVal + 1) {
						stack.PushBack(Location{col + 1, row})
					}
				}
				// down
				if row < len((*tmap))-1 {
					bVal := (*tmap)[row+1][col]
					if bVal == (currVal + 1) {
						stack.PushBack(Location{col, row + 1})
					}
				}
			}

			(*theads)[key] = len(paths)
		}

	}(&trailheads, topographicMap)

	// - score trailhead

	for _, score := range trailheads {
		scoresSum += score
	}

	return scoresSum, nil
}
