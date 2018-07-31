pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut n = n;
    let mut steps = 0;
    loop {
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
            steps += 1;
        } else {
            n = n * 3 + 1;
            steps += 1;
        }
    }

    Some(steps)
}
