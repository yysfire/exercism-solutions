/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code = code.to_owned();
    code.retain(|x| x != ' ');
    if code.len() < 2 {
        return false;
    }

    if !code.chars().all(|x| x.is_ascii_digit()) {
        return false;
    }

    let sum: u32 = code.chars()
        .map(|x| x.to_digit(10).unwrap())
        .rev()
        .enumerate()
        .map(|(i, x)|{ if i % 2 == 1 { x * 2 } else { x } })
        .map(|x|{ if x > 9 { x - 9 } else { x } })
        .sum();

    sum % 10 == 0
}
