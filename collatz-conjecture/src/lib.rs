pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    } else if n == 1 {
        return Some(0);
    }

    if n % 2 == 0 {
        Some(1 + collatz(n / 2).unwrap())
    } else {
        Some(1 + collatz(n * 3 + 1).unwrap())
    }
}
