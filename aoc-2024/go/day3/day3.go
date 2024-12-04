package day3

import (
	"container/list"
	"errors"
	"strconv"
	"strings"
	"text/scanner"
)

func SumOfMultiplication(text []string) (int, error) {
	sum := 0
	muls := list.New()

	if len(text) == 0 {
		return sum, errors.New("Empty Data!")
	}

	for _, line := range text {

		var s scanner.Scanner
		s.Init(strings.NewReader(line))
		s.Filename = "testfile"
		s.Whitespace ^= 1 << ' '

		var (
			tokens = make([]string, 0)
		)

		// assign the token to the array
		for tok := s.Scan(); tok != scanner.EOF; tok = s.Scan() {
			tokens = append(tokens, s.TokenText())
		}

		for idx, tok := range tokens {
			if strings.HasSuffix(tok, "mul") && (idx+5) < len(tokens) {
				openPar := tokens[idx+1]
				op1 := tokens[idx+2]
				cmm := tokens[idx+3]
				op2 := tokens[idx+4]
				closePar := tokens[idx+5]

				if ValidExpression(openPar, op1, cmm, op2, closePar) {
					v1, err1 := strconv.Atoi(op1)
					v2, err2 := strconv.Atoi(op2)
					if err1 == nil && err2 == nil {
						muls.PushBack(v1 * v2)
					}
				}
			}
		}
	}

	// sum the multiplications
	for val := muls.Front(); val != nil; val = val.Next() {
		if v, ok := val.Value.(int); ok {
			sum += v
			continue
		}
		return sum, errors.New("Could not convert value to int")
	}

	return sum, nil
}

func ValidExpression(openPar, op1, cmm, op2, closePar string) bool {
	truthT := make([]bool, 5)

	truthT[0] = strings.Compare(openPar, "(") == 0
	if _, err := strconv.Atoi(op1); err == nil {
		truthT[1] = true
	}
	truthT[2] = strings.Compare(cmm, ",") == 0
	if _, err := strconv.Atoi(op2); err == nil {
		truthT[3] = true

	}
	truthT[4] = strings.Compare(closePar, ")") == 0

	for _, b := range truthT {
		if !b {
			return false
		}
	}

	return true
}
