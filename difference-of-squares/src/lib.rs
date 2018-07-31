pub fn square_of_sum(n: usize) -> usize {
    let sum = (0..=n).fold(0, |acc, x| acc + x);
    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
    (0..=n).map(|x| x * x).fold(0, |acc, x| acc + x)
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
