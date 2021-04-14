pub fn factors(n: u64) -> Vec<u64> {
    if n == 0 || n == 1 {
        return vec![];
    }

    let mut m = n;
    let mut result = vec![];
    let mut d = 2;
    loop {
        if m % d != 0 {
            d += 1;
            continue;
        }
        result.push(d);
        m /= d;
        if m == 1 {
            break;
        }
    }

    result
}
