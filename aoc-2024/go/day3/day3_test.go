package day3_test

import (
	. "aoc/day3"
	"aoc/utils"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestValidExpression(t *testing.T) {
	actual := ValidExpression("(", "234", ",", "456", ")")
	assert.True(t, actual)
	assert.False(t, ValidExpression("(", "\n", ",", "456", ")"))
	assert.True(t, ValidExpression("(", "0", ",", "1", ")"))
	assert.False(t, ValidExpression(")", "34", ",", "23", "("))
}

func TestSumOfMultiplication(t *testing.T) {
	text, err := utils.ReadFile("./test_data.txt")
	if err != nil {
		t.Errorf("Error while reading file: %v", err)
	}

	expected := 161
	actual, err := SumOfMultiplication(text)
	if err != nil {
		t.Errorf("Error while calculating sum of multiplication: %v", err)
	}

	assert.Equal(t, expected, actual)
}

func TestSumOfMultiplicationV2(t *testing.T) {
	text, err := utils.ReadFile("./test_data2.txt")
	if err != nil {
		t.Errorf("Error while reading file: %v", err)
	}

	expected := 48
	actual, err := SumOfMultiplicationV2(text)
	if err != nil {
		t.Errorf("Error while calculating sum of multiplication: %v", err)
	}

	assert.Equal(t, expected, actual)
}
