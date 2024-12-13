package day10

import (
	"errors"
	"fmt"
	// "fmt"
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
	func(theads *map[Location]int) {
		//
		for k, _ := range *theads {
			//
			fmt.Println(k) // TODO: Remove me
			fmt.Printf("Row: %d, col: %d\n", k.y, k.x)

		}

	}(&trailheads)

	// - score trailhead
	//

	for _, score := range trailheads {
		scoresSum += score
	}

	return scoresSum, nil //TODO: replace the with the return variable
}
