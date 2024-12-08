package day6_test

import (
	. "aoc/day6"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestVisitedPositions(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("GiveNoDataItShouldError", func(t *testing.T) {
		_, err := VisitedPositions([]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenMapItShouldReturn", func(t *testing.T) {
		actual, err := VisitedPositions(data)
		assert.Nil(t, err)
		expected := 41
		assert.Equal(t, expected, actual)
	})
}
