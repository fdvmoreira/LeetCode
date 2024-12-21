package day14_test

import (
	. "aoc/day14"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSafetyFactor(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	//
	t.Run("ShouldErrorWhenNoData", func(t *testing.T) {
		_, err := SafetyFactor(&[]string{})
		assert.NotNil(t, err)
	})

	t.Run("GivenInputItShouldCalculateTheSafetyFactor", func(t *testing.T) {
		actual, err := SafetyFactor(&data)
		assert.Nil(t, err)

		expected := 12
		assert.Equal(t, expected, actual)
	})
}
