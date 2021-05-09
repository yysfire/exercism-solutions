// Package proverb provides Proverb function.
package proverb

import "fmt"

// Proverb generate the relevant proverb according to a list of inputs.
func Proverb(rhyme []string) []string {
	var pvb []string
	for i, word := range rhyme {
		if i < len(rhyme)-1 {
			pvb = append(pvb, fmt.Sprintf("For want of a %s the %s was lost.", word, rhyme[i+1]))
		} else {
			pvb = append(pvb, fmt.Sprintf("And all for the want of a %s.", rhyme[0]))
		}
	}
	return pvb
}
