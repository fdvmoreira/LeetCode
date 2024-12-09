package day7_test

import (
	. "aoc/day7"
	"aoc/utils"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

type EquationTable struct {
	target      int
	equationVal []int
	expected    bool
}

func TestCalculateEquation(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err, "Error Reading the file")

	t.Run("GiveNoDataItShouldError", func(t *testing.T) {
		_, err = CalculateCalibration([]string{})
		assert.NotNil(t, err, "Should have errored!")
	})

	t.Run("ShouldCalculateTheSumOfPossibleEquation", func(t *testing.T) {
		actual, err := CalculateCalibration(data)
		assert.Nil(t, err, "Error was not expected")

		const expected = 3749
		assert.Equal(t, actual, expected)
	})

}

func TestEvaluateEquation(t *testing.T) {
	data := []EquationTable{
		{190, []int{10, 19}, true},
		{3267, []int{81, 40, 27}, true},
		{83, []int{17, 5}, false},
		{156, []int{15, 6}, false},
		{7290, []int{6, 8, 6, 15}, false},
		{161011, []int{16, 10, 13}, false},
		{192, []int{17, 8, 14}, false},
		{21037, []int{9, 7, 18, 13}, false},
		{292, []int{11, 6, 16, 20}, true},
	}

	for _, v := range data {
		t.Run(fmt.Sprintf("Given%vAnd%vItShouldReturn%v", v.equationVal, v.target, v.expected), func(t *testing.T) {
			actual := EvaluateEquation(&v.equationVal, v.target)
			assert.Equal(t, actual, v.expected)
		})
	}
}
