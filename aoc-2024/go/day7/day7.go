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

		if EvaluateEquation(&nums, float64(result)) {
			sum += result
		}
	}

	return sum, nil
}

func EvaluateEquation(equation *[]int, target float64) bool { // TODO: memoization required to make it more efficient
	aLen := len(*equation)
	if (target == 0 || (target < 0.02 && target > -0.02)) && aLen == 0 { //REVISE: Floating point does not returning accurate numbers and it mess the calculation. So, change the algorithm
		return true
	}

	if target < 0 || aLen == 0 {
		return false
	}

	rem1 := target - float64((*equation)[0])
	rem2 := target / float64((*equation)[0])

	slice := (*equation)[1:]

	if EvaluateEquation(&slice, rem1) || EvaluateEquation(&slice, rem2) {
		return true
	}

	return false
}
