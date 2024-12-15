package day11_test

import (
	. "aoc/day11"
	"testing"

	"github.com/stretchr/testify/assert"
)

type Testa struct {
	stones string
	blink  int
	want   int
}

func TestCountStones(t *testing.T) {

	testData := []Testa{
		{"125 17", 1, 3},
		{"125 17", 3, 5},
		{"125 17", 5, 13},
		{"125 17", 6, 22},
		{"125 17", 25, 55312},
	}

	t.Run("GivenNoDataItShouldError", func(t *testing.T) {
		_, err := CountStones("", 1)
		assert.NotNil(t, err)
	})
	for _, td := range testData {

		t.Run("GivenDataItShouldCountStones", func(t *testing.T) {
			actual, err := CountStones(td.stones, td.blink)
			assert.Nil(t, err)

			assert.Equal(t, td.want, actual)
		})
	}
}
