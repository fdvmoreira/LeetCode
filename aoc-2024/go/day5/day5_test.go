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

func TestIsPagesOrdered(t *testing.T) { // TODO: Add Fuzz testing or change to table/data drive test
	var (
		p1Expected = []int{75, 47, 61, 53, 29}
		p2Expected = []int{97, 61, 53, 29, 13}
		p3Expected = []int{75, 29, 13}
		p4Expected = []int{75, 97, 47, 61, 53}
		p5Expected = []int{61, 13, 29}
		p6Expected = []int{97, 13, 75, 29, 47}
	)

	assert.True(t, IsPageOrdered(p1Expected))
	assert.True(t, IsPageOrdered(p2Expected))
	assert.True(t, IsPageOrdered(p3Expected))
	assert.False(t, IsPageOrdered(p4Expected))
	assert.False(t, IsPageOrdered(p5Expected))
	assert.False(t, IsPageOrdered(p6Expected))
}
