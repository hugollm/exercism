use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::<&'a str>::new();
    let lowercased_word = word.to_lowercase();
    let sorted_word = sort_word(&lowercased_word);
    for candidate in possible_anagrams {
        let lowercased_candidate = candidate.to_lowercase();
        if lowercased_candidate == lowercased_word {
            continue;
        }
        let sorted_candidate = sort_word(&lowercased_candidate);
        if sorted_word == sorted_candidate {
            anagrams.insert(candidate);
        }
    }
    anagrams
}

fn sort_word(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.iter().collect()
}
