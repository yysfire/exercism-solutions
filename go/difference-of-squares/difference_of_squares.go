//Package diffsquares provides functions to calculate the square of the sum and
//the sum of the squares of the first N natural numbers, and the difference between them.
package diffsquares

//SquareOfSum return the square of the sum of the first n natural numbers.
func SquareOfSum(n int) int {
	sum := 0
	for i := 1; i <= n; i++ {
		sum += i
	}
	return sum * sum
}

//SumOfSquares return the sum of the squares of the first n natural numbers.
func SumOfSquares(n int) int {
	sum := 0
	for i := 1; i <= n; i++ {
		sum += i * i
	}
	return sum
}

//Difference return the difference between the square of the sum and
//the sum of the squares of the first n natural numbers.
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
