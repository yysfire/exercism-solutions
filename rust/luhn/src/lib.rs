/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.chars().filter(|x| !x.is_whitespace()).collect();
    if code.len() < 2 {
        return false;
    }

    if code.chars().any(|x| !x.is_ascii_digit()) {
        return false;
    }

    code.chars()
        .filter_map(|x| x.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, x)| (x << (i & 1)) - if i & 1 == 1 && x > 4 { 9 } else { 0 })
        .sum::<u32>() % 10 == 0
}
