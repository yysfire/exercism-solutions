use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    for c in candidate.chars() {
        if c == ' ' || c == '-' {
            continue;
        }
        let ch = c.to_ascii_lowercase();
        if !ch.is_ascii_alphabetic() || set.contains(&ch) {
            return false;
        }
        set.insert(ch);
    }

    true
}
