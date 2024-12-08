package day5

import (
	"errors"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

func SumMiddlePages(data []string) (int, error) {
	sum := 0
	if len(data) == 0 {
		fmt.Printf("No data")
		return sum, nil
	}

	var (
		rules   = make(map[string]bool)
		updates [][]int
	)

	_ = rules

	sectionPlitIndex := slices.Index(data, "")

	if sectionPlitIndex == -1 {
		return sum, errors.New("Section Devision/Split Error")
	}

	// split the data into rules and updates
	for i := 0; i <= sectionPlitIndex-1; i++ {
		rules[strings.TrimSpace(data[i])] = false
	}

	for _, line := range data[sectionPlitIndex+1:] {

		// transform the string into array of numbers
		pages := func(str *string) (arr []int) {
			tmpNum := strings.Split(*str, ",")
			for _, v := range tmpNum {
				num, err := strconv.Atoi(v)
				if err != nil {
					fmt.Printf("Error: %v\n", err.Error())
				}
				arr = append(arr, num)
			}

			return

		}(&line)

		updates = append(updates, pages)
	}

	for _, update := range updates {
		if IsPageOrdered(update, &rules) {
			middleIndex := int(math.Floor(float64(len(update) / 2)))
			sum += update[middleIndex]
		}
	}

	return sum, nil
}

func IsPageOrdered(pages []int, m *map[string]bool) bool {
	if len(pages) == 0 {
		return false
	}

	for i, page := range pages {
		for j := i + 1; j < len(pages); j++ {

			key := fmt.Sprintf("%v|%v", pages[j], page)
			_, ok := (*m)[key]
			if ok {
				return false
			}
		}
	}

	return true
}
