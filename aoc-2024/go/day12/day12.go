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
	// count the marked location (as these should ne unique for each group). This can be done linearly by using a map
	// ---
	// Another alternative
	// ---
	// Scan line by line  and determine the present group, the start and end, check if the previous group ended
	//

	return totalFencePrice, nil
}
