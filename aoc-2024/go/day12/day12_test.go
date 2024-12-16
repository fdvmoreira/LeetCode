package day12_test

import (
	. "aoc/day12"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCalculateFencePrice(t *testing.T) {
	garden, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("GivenNoDataItShouldError", func(t *testing.T) {
		_, err := CalculateFencePrice(&[]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenDataItShouldCalculateTotalFencePrice", func(t *testing.T) {
		actual, err := CalculateFencePrice(&garden)
		assert.Nil(t, err)

		expected := 1930
		assert.Equal(t, expected, actual)
	})
}
