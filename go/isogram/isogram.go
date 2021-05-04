//Package isogram provides a function to determine if a word or phrase is an isogram.
//An isogram (also known as a "nonpattern word") is a word or phrase without a repeating letter,
//however spaces and hyphens are allowed to appear multiple times.
package isogram

import "unicode"

//IsIsogram determine if a word or phrase is an isogram.
func IsIsogram(phrase string) bool {
	repeatMap := make(map[rune]bool)
	for _, r := range phrase {
		if r == '-' || unicode.IsSpace(r) {
			continue
		}
		r = unicode.ToLower(r)
		if _, ok := repeatMap[r]; ok {
			return false
		}
		repeatMap[r] = true
	}
	return true
}
