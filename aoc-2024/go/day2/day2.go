package day2

import (
	// "fmt"
	"fmt"
	"strconv"
	"strings"
)

type Order int

const (
	Increasing Order = iota
	Decreasing
)

func CountSafeLevels(reports []string) (int, error) {
	count := 0

	for _, report := range reports {

		safe := func(report string) bool {
			levels := strings.Fields(report)
			var iLevels []int = make([]int, len(levels))

			for idx, v := range levels {
				val, err := strconv.Atoi(v)
				if err != nil {
					panic(err)
				}
				iLevels[idx] = val
			}

			var initialOrder = Increasing
			const maxAdjecentLevelDistance = 3
			const minAdjacentLevelDistance = 1

			// determine the correct order in the list
			if first, second := iLevels[0], iLevels[1]; first > second {
				initialOrder = Decreasing
			}

			for i := 1; i < len(iLevels); i++ {
				lastLevel, currentLevel := iLevels[i-1], iLevels[i]
				currentAdjacentLevelDistance := currentLevel - lastLevel

				if initialOrder == Decreasing {
					currentAdjacentLevelDistance = lastLevel - currentLevel
				}

				currentOrder := initialOrder

				// determine if the order is ascending or descending according to the first and second level
				if firstLevel, secondLevel := iLevels[i-1], iLevels[i]; firstLevel > secondLevel {
					currentOrder = Decreasing
				} else {
					currentOrder = Increasing
				}

				if (currentAdjacentLevelDistance < minAdjacentLevelDistance) || (currentAdjacentLevelDistance > maxAdjecentLevelDistance) || (currentOrder != initialOrder) {
					return false
				}
			}

			return true
		}(report)

		if safe {
			count += 1
		}
	}

	return count, nil
}

func CountSafeLevelsWithTolerance(reports []string) (int, error) {
	count := 0

	for _, report := range reports {

		safe := func(report string) bool {
			levels := strings.Fields(report)
			var iLevels []int = make([]int, len(levels))

			for idx, v := range levels {
				val, err := strconv.Atoi(v)
				if err != nil {
					panic(err)
				}
				iLevels[idx] = val
			}

			var initialOrder = Increasing
			const maxAdjecentLevelDistance = 3
			const minAdjacentLevelDistance = 1

			// determine the correct order in the list
			if first, second := iLevels[0], iLevels[1]; first > second {
				initialOrder = Decreasing
			}

			badLevelCounter := 0

			for i := 1; i < len(iLevels); i++ {
				lastLevel, currentLevel := iLevels[i-1], iLevels[i]
				currentAdjacentLevelDistance := currentLevel - lastLevel

				if initialOrder == Decreasing {
					currentAdjacentLevelDistance = lastLevel - currentLevel
				}

				currentOrder := initialOrder

				// determine if the order is ascending or descending according to the first and second level
				if firstLevel, secondLevel := iLevels[i-1], iLevels[i]; firstLevel > secondLevel {
					currentOrder = Decreasing
				} else {
					currentOrder = Increasing
				}

				if (currentAdjacentLevelDistance < minAdjacentLevelDistance) || (currentAdjacentLevelDistance > maxAdjecentLevelDistance) || (currentOrder != initialOrder) {
					badLevelCounter += 1
					//TODO: Remove the bad level once and reiterate
					fmt.Printf("x")
				}
			}

			return (badLevelCounter <= 1)
		}(report)

		if safe {
			count += 1
		}
	}

	return count, nil
}
