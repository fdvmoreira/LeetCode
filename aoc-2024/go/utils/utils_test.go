package utils_test

import (
	. "aoc/utils"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReadFile(t *testing.T) {
	data, err := ReadFile("input.txt")
	if err != nil {
		t.Errorf("Error: %v", err)
	}

	// trim the spaces in between the strings
	for i := range data {
		data[i] = strings.TrimSpace(strings.Join(strings.Split(data[i], " "), ""))
	}
	assert.EqualValues(t, "34", data[0])
	assert.EqualValues(t, "43", data[1])
	assert.EqualValues(t, "25", data[2])
	assert.EqualValues(t, "13", data[3])
	assert.EqualValues(t, "39", data[4])
	assert.EqualValues(t, "33", data[5])
}
