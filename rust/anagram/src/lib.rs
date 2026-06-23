use core::iter::Iterator;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = sort_word(word);
    let answers: HashSet<&str> = possible_anagrams
        .iter()
        .copied()
        .filter(|pos| (word != *pos) && (sorted_word == sort_word(pos)))
        .collect();
    answers
}

pub fn sort_word(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars
}
