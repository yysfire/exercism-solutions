pub fn series(digits: &str, len: usize) -> Vec<String> {
    let str_len = digits.len();
    if len == 0 {
        return vec!["".to_string(); str_len + 1];
    }

    let mut result = Vec::new();
    for i in 0..str_len {
        if i + len <= str_len {
            result.push(digits[i..i+len].to_string());
        }
    }

    result
}
