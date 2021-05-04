//Package scrabble provides a function to compute the Scrabble score for a word
package scrabble

import (
	"unicode"
)

var pointMap = map[int]string{
	1:  "AEIOULNRST",
	2:  "DG",
	3:  "BCMP",
	4:  "FHVWY",
	5:  "K",
	8:  "JX",
	10: "QZ",
}
var letterMap = make(map[rune]int)

func init() {
	for point, letters := range pointMap {
		for _, r := range letters {
			letterMap[r] = point
		}
	}
}

//Score return the Scrabble score for the input word
func Score(input string) int {
	score := 0
	for _, r := range input {
		c := unicode.ToUpper(r)
		if point, ok := letterMap[c]; ok {
			score += point
		}
	}
	return score
}
