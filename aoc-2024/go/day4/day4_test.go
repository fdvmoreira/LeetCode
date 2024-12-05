package day4_test

import (
	. "aoc/day4"
	"aoc/utils"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestXMasWordSearch(t *testing.T) {
	data, err := utils.ReadFile("./test_data.txt")
	if err != nil {
		fmt.Printf("error: %v\n", err.Error())
	}

	const expected int = 18
	actual, err := XMasWordSearch(data)
	if err != nil {
		fmt.Printf("error: %v\n", err.Error())
	}
	assert.Equal(t, expected, actual)
}

func TestXMasWordSearchV2(t *testing.T) {
	data, err := utils.ReadFile("./test_data2.txt")
	if err != nil {
		fmt.Printf("error: %v\n", err.Error())
	}

	const expected int = 4
	actual, err := XMasWordSearch(data)
	if err != nil {
		fmt.Printf("error: %v\n", err.Error())
	}
	assert.Equal(t, expected, actual)
}
