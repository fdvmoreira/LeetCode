package utils

import (
	"os"
	"strings"
)

func ReadFile(path string) ([]string, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	tmp := strings.Split(string(data), "\n")

	// Remove the last element if it is empty
	if tmp[len(tmp)-1] == "" {
		tmp = tmp[:len(tmp)-1]
	}
	return tmp, nil
}
