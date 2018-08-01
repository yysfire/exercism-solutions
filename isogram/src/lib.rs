use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut candidate = candidate.to_ascii_lowercase();
    candidate.retain(|c| c != '-' && c != ' ');

    let mut set = HashSet::new();
    for c in candidate.chars() {
        if !c.is_ascii_alphabetic() || set.contains(&c) {
            return false;
        }
        set.insert(c);
    }

    true
}
