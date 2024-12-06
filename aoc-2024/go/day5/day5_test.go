package day5_test

import (
	. "aoc/day5"
	"aoc/utils"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSumMiddlePages(t *testing.T) {
	data, err := utils.ReadFile("test_data.txt")
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err.Error())
	}

	expected := 143
	actual, err := SumMiddlePages(data)
	if err != nil {
		t.Errorf("Error: %v", err)
	}
	assert.Equal(t, expected, actual)
}
