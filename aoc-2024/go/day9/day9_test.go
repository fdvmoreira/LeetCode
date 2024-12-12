package day9_test

import (
	. "aoc/day9"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFSChecksum(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("EmptyDataShouldError", func(t *testing.T) {
		_, err := FSChecksum(&[]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenDataItShouldReturnChecksum", func(t *testing.T) {
		actual, err := FSChecksum(&data)
		assert.Nil(t, err)
		expected := 1928
		assert.Equal(t, expected, actual)
	})
}
