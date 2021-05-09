// Package summultiples provides a function to calculate the sum of
// all the unique multiples of particular numbers up to but not including the given number.
package summultiples

// SumMultiples return the sum of all the unique multiples of
// particular numbers up to but not including the given number.
func SumMultiples(limit int, divisors ...int) int {
	multiples := make(map[int]bool)
	for _, dv := range divisors {
		if dv == 0 {
			continue
		}

		for i := 1; i <= limit/dv; i++ {
			m := i * dv
			if m < limit {
				multiples[m] = true
			}
		}
	}

	sum := 0
	for m := range multiples {
		sum += m
	}
	return sum
}
