package day8_test

import (
	. "aoc/day8"
	"aoc/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountAntinode(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	assert.Nil(t, err)

	t.Run("ShouldError", func(t *testing.T) {
		m := []string{}
		_, err := CountAntinode(&m)
		assert.NotNil(t, err)
	})

	t.Run("GivenDataShouldCountAntinode", func(t *testing.T) {
		actual, err := CountAntinode(&data)
		assert.Nil(t, err)

		const expect = 14
		assert.Equal(t, expect, actual)
	})
}
