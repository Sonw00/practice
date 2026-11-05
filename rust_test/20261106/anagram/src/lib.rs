use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    possible_anagrams.iter().filter(|candidate| is_anagram(word, candidate)).collect()
}

fn is_anagram(target: &str, candidate: &str) -> bool {
    if target.len() != candidate.len() {
        return false;
    }

    target.to_lowercase().chars().all(|t| candidate.to_lowercase().contains(t))
}