package day9

import (
	"errors"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func FSChecksum(diskMap *[]string) (int, error) {
	checksum := 0
	if len(*diskMap) == 0 {
		return checksum, errors.New("No Data")
	}

	//decode the string
	var decodedDiskMap strings.Builder
	line := strings.Split((*diskMap)[0], "")
	fmt.Println((*diskMap))

	for i := 0; i < len(line); i += 2 {
		var numSpace = 0
		numBlocks, err := strconv.Atoi(line[i])
		if err != nil {
			return checksum, fmt.Errorf("Converting %v to INT failed", line[i])
		}

		id := strconv.Itoa(i / 2)

		decodedDiskMap.WriteString(strings.Repeat(id, numBlocks))

		if i+1 < len(line) {
			numSpace, err = strconv.Atoi(line[i+1])
			if err != nil {
				return checksum, fmt.Errorf("Converting %v to INT failed", line[i])
			}
		}
		decodedDiskMap.WriteString(strings.Repeat(".", numSpace))
	}

	// compress
	compressedDiskMap := strings.Split(decodedDiskMap.String(), "")
	startP, endP := 0, len(compressedDiskMap)-1

	for startP != endP {

		if strings.Contains(compressedDiskMap[startP], ".") {
			for strings.Contains(compressedDiskMap[endP], ".") {
				endP -= 1
			}
			compressedDiskMap[startP] = compressedDiskMap[endP]
			compressedDiskMap[endP] = "."
		}

		startP += 1
	}

	//checksum
	lastIDIndex := slices.Index(compressedDiskMap, ".")
	for i := 0; i != lastIDIndex; i++ {
		id, err := strconv.Atoi(compressedDiskMap[i])

		if err != nil {
			return 0, fmt.Errorf("Error convering %v to INT\nError: %v", id, err.Error())
		}

		checksum = checksum + (i * id)
	}

	return checksum, nil
}
