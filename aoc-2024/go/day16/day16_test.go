package day16_test

import (
	. "aoc/day16"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMazeCost(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("GivenNoDataItShouldError", func(t *testing.T) {
		_, err := MazeCost(&[]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenTheDataItShouldCalculateTheCost", func(t *testing.T) {
		actual, err := MazeCost(&data)
		assert.Nil(t, err)

		expected := 7036
		assert.Equal(t, expected, actual)
	})
}
