//Package scrabble provides a function to compute the Scrabble score for a word
package scrabble

import (
	"unicode"
)

//Score return the Scrabble score for the input word
func Score(input string) int {
	score := 0
	for _, r := range input {
		c := unicode.ToUpper(r)
		switch c {
		case 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T':
			score += 1
		case 'D', 'G':
			score += 2
		case 'B', 'C', 'M', 'P':
			score += 3
		case 'F', 'H', 'V', 'W', 'Y':
			score += 4
		case 'K':
			score += 5
		case 'J', 'X':
			score += 8
		case 'Q', 'Z':
			score += 10
		}
	}
	return score
}
