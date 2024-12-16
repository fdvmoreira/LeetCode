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

	// TODO:
	// Find start of each group
	// Find the height and width of each group
	// NOTE: Mark the visited location to prevent double
	// count the marked location (as these should ne unique for each group)

	return totalFencePrice, nil
}
