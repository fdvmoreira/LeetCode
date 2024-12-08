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

type TableTest struct {
	input    []int
	expected bool
}

func TestIsPagesOrdered(t *testing.T) { // TODO: Add Fuzz testing or change to table/data drive test
	rules := map[string]bool{
		"47|53": false,
		"97|13": false,
		"97|61": false,
		"97|47": false,
		"75|29": false,
		"61|13": false,
		"75|53": false,
		"29|13": false,
		"97|29": false,
		"53|29": false,
		"61|53": false,
		"97|53": false,
		"61|29": false,
		"47|13": false,
		"75|47": false,
		"97|75": false,
		"47|61": false,
		"75|61": false,
		"47|29": false,
		"75|13": false,
		"53|13": false,
	}

	testData := []TableTest{{[]int{75, 47, 61, 53, 29}, true},
		{[]int{97, 61, 53, 29, 13}, true},
		{[]int{75, 29, 13}, true},
		{[]int{75, 97, 47, 61, 53}, false},
		{[]int{61, 13, 29}, false},
		{[]int{97, 13, 75, 29, 47}, false}}

	for _, test := range testData {
		actual := IsPageOrdered(test.input, &rules)
		assert.Equal(t, actual, test.expected)
	}
}
