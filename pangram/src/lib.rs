use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let set: HashSet<char> = sentence.to_ascii_lowercase().chars().collect();
    (97u8..123).filter(|x| set.contains(&char::from(*x))).count() == 26
}
