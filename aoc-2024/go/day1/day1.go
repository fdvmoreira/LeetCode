package day1

import (
	"math"
	"slices"
	"strconv"
	"strings"
)

func TotalDistance(data []string) int {

	leftList, rightList := func(data []string) ([]int, []int) {

		first := make([]int, len(data))
		second := make([]int, len(data))

		for idx, v := range data {
			row := strings.Fields(v) // split array and ignore the whitespace
			var err error

			first[idx], err = strconv.Atoi(row[0])
			if err != nil {
				panic(err)
			}
			second[idx], err = strconv.Atoi(row[1])
			if err != nil {
				panic(err)
			}
		}

		slices.Sort(first)
		slices.Sort(second)

		return first, second
	}(data)

	sum := 0

	for i := 0; i < len(data); i++ {
		// NOTE: abs is used because the diff might be negative
		absDiff := math.Abs(float64(leftList[i]) - float64(rightList[i]))
		sum += int(absDiff)
	}

	return sum
}
