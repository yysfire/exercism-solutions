package hamming

import (
	"errors"
	"strings"
)

//Distance calculate the Hamming distance between two DNA strands.
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("length of a not equal to b")
	}

	distance := 0
	bReader := strings.NewReader(b)
	for _, ach := range a {
		bch, _, err := bReader.ReadRune()
		if err != nil {
			return 0, err
		}
		if ach != bch {
			distance++
		}
	}

	return distance, nil
}
