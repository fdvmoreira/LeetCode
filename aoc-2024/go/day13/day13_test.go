package day13_test

import (
	. "aoc/day13"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSumSmallestTokenAmount(t *testing.T) {
	puzzle, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("GivenNoDataItShouldError", func(t *testing.T) {
		_, err := SumSmallestTokenAmount(&[]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenDataGetTheSmallestAmoutOfTokenToWinPrizes", func(t *testing.T) {
		actual, err := SumSmallestTokenAmount(&puzzle)
		assert.Nil(t, err)

		expected := 480
		assert.Equal(t, expected, actual)
	})
}
