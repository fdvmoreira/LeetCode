package day7

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

func CalculateCalibration(data []string) (int, error) {
	if len(data) == 0 {
		return 0, errors.New("No Data")
	}
	sum := 0

	for _, line := range data {
		tmp := strings.Split(line, ":")

		// get the result
		result, err := strconv.Atoi(tmp[0])
		if err != nil {
			return sum, fmt.Errorf("Couldn't convert %q to int]", tmp[0])
		}

		//get the string of numbers to slice of ints
		var nums []int
		for _, v := range strings.Fields(tmp[1]) {
			num, err := strconv.Atoi(v)
			if err != nil {
				return sum, fmt.Errorf("Error converting string to num")
			}
			nums = append(nums, num) // NOTE: Not efficient because the slice is being resized with every iteration
		}

		if EvaluateEquation(&nums, result) {
			sum += 1
		}
	}

	return sum, nil
}

func EvaluateEquation(equation *[]int, target int) bool {
	if len(*equation) == 0 {
		return false
	}

	//

	return true
}
