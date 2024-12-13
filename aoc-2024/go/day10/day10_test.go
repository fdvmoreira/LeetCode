package day10_test

import (
	. "aoc/day10"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSumTrailheadsScore(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("ShouldFaildWhenNoData", func(t *testing.T) {
		_, err := SumTrailheadsScore(&[]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenDataItShouldReturnExpectedValue", func(t *testing.T) {
		actual, err := SumTrailheadsScore(&data)
		assert.Nil(t, err)
		expected := 36
		assert.Equal(t, expected, actual)
	})
}
