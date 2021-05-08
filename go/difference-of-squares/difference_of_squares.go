//Package diffsquares provides functions to calculate the square of the sum and
//the sum of the squares of the first N natural numbers, and the difference between them.
package diffsquares

//SquareOfSum return the square of the sum of the first n natural numbers.
func SquareOfSum(n int) int {
	sum := (n + 1) * n / 2
	return sum * sum
}

//SumOfSquares return the sum of the squares of the first n natural numbers.
//https://en.wikipedia.org/wiki/Summation#Powers_and_logarithm_of_arithmetic_progressions
func SumOfSquares(n int) int {
	return n * (n + 1) * (2*n + 1) / 6
}

//Difference return the difference between the square of the sum and
//the sum of the squares of the first n natural numbers.
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
