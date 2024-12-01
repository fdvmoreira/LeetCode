package day1_test

import (
	. "aoc/day1"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTotalDistance(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	if assert.NoError(t, err) {

		actual := TotalDistance(data)
		const expected = 11

		assert.Equal(t, expected, actual)
	}
}

func TestSimilarityScore(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	if assert.NoError(t, err) {

		actual := SimilarityScore(data)
		const expected = 31

		assert.Equal(t, expected, actual)
	}
}
