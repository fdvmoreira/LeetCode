package day11

import (
	"errors"
	"fmt"
	"log/slog"
	"strconv"
	"strings"
)

func CountStones(stones string, blink int) (int, error) {
	stonesCount := 0
	if len(stones) == 0 {
		slog.Error("No Data", slog.Int("Size", len(stones)))
		return stonesCount, errors.New("No Data")
	}

	var resStones []string = strings.Fields(stones)

	for i := 0; i < blink; i++ {

		tempStones := resStones
		resStones = nil

		for _, v := range tempStones {
			if v == "0" {
				resStones = append(resStones, "1")
			}

			strLen := len(v)
			if strLen > 0 && strLen%2 == 0 {
				a, b := v[:strLen/2], v[strLen/2:]
				if strings.HasPrefix(b, "0") {
					tmp, err := strconv.Atoi(b)
					b = fmt.Sprint(tmp)
					if err != nil {
						slog.Error("Conversion went wrong!", slog.String("Error", err.Error()))
						return 0, fmt.Errorf("Conversion failed;\tMsg:%v", err.Error())
					}
				}
				resStones = append(resStones, a, b)
			}

			if v != "0" && strLen%2 == 1 {
				i, err := strconv.Atoi(v)
				if err != nil {
					slog.Error("Conversion went wrong!", slog.String("Error", err.Error()))
					return 0, fmt.Errorf("Conversion failed;\tMsg:%v", err.Error())
				}
				resStones = append(resStones, fmt.Sprint(i*2024))
			}
		}
	}

	stonesCount = len(resStones)

	return stonesCount, nil
}
