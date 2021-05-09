pub fn square_of_sum(n: usize) -> usize {
    let sum = (0..=n).sum::<usize>();
    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
    (0..=n).map(|x| x * x).sum::<usize>()
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
