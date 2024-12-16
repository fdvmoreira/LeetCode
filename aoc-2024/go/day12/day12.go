package day12

import (
	"errors"
	"log/slog"
)

func CalculateFencePrice(garden *[]string) (int, error) {
	totalFencePrice := 0

	if len(*garden) == 0 {
		slog.Error("No Data Given")
		return totalFencePrice, errors.New("No Data")
	}

	return totalFencePrice, nil
}
