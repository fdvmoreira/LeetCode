package day15_test

import (
	. "aoc/day15"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSumBoxesGPSCoords(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("GivenNoDataItShouldError", func(t *testing.T) {
		_, err := SumBoxesGPSCoordinates(&[]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenDataItShouldSumBoxesGPSCoords", func(t *testing.T) {
		actual, err := SumBoxesGPSCoordinates(&data)
		assert.Nil(t, err)

		const expected = 10092
		assert.Equal(t, expected, actual)
	})

}
