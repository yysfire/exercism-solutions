use std::collections::HashSet;

///Given a word and a list of candidates, select the sublist of anagrams of the given word.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();

    let mut word_sorted: Vec<char> = word.to_lowercase().chars().collect();
    word_sorted.sort_unstable();

    for anagram in possible_anagrams {
        if anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut anagram_sorted: Vec<char> = anagram.to_lowercase().chars().collect();
        anagram_sorted.sort_unstable();

        if word_sorted == anagram_sorted {
            result.insert(anagram);
        }
    }

    result
}
