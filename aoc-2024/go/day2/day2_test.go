package day2_test

import (
	. "aoc/day2"
	. "aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountSafeLevels(t *testing.T) {
	data, err := ReadFile("./test_data.txt")
	if !assert.NoError(t, err) {
		assert.Fail(t, err.Error())
	}

	const expected = 3
	actual, err := CountSafeLevels(data)
	if assert.NoError(t, err) {
		assert.Equal(t, expected, actual)
	}
}
