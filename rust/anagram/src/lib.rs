use std::collections::HashSet;

///Given a word and a list of candidates, select the sublist of anagrams of the given word.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();

    let word_string = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_string.chars().collect();
    word_sorted.sort_unstable();

    for anagram in possible_anagrams {
        let anagram_string = anagram.to_lowercase();
        if anagram_string == word_string {
            continue;
        }

        let mut anagram_sorted: Vec<char> = anagram_string.chars().collect();
        anagram_sorted.sort_unstable();

        if word_sorted == anagram_sorted {
            result.insert(anagram);
        }
    }

    result
}
