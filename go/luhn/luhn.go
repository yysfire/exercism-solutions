//Package luhn provides a function to determine whether or not a number is valid per the Luhn formula.
//https://en.wikipedia.org/wiki/Luhn_algorithm
package luhn

import (
	"strconv"
)

//Valid return true if ident is valid per the Luhn formula
func Valid(ident string) bool {
	count := 0
	sum := 0
	for i := len(ident) - 1; i >= 0; i-- {
		c := string(ident[i])
		if c == " " {
			continue
		}

		d, err := strconv.Atoi(c)
		if err != nil {
			return false
		}
		if count%2 == 1 {
			d *= 2
			if d > 9 {
				d -= 9
			}
		}
		sum += d
		count++
	}

	if count <= 1 {
		return false
	}

	return sum%10 == 0
}
