use std::collections::HashSet;

pub fn anagrams_for<'a>(word: & str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().filter(|candidate| is_anagram(word, candidate)).copied().collect()
}

fn is_anagram(target: &str, candidate: &str) -> bool {

    if target.len() != candidate.len() {
        return false;
    }

    let target_string = target.to_lowercase();
    let mut candidate_string = candidate.to_lowercase();
    
    if target_string == candidate_string {
        return false;
    }

    for target_char in target_string.chars() {
        candidate_string = candidate_string.replacen(target_char, "", 1);
    }

    candidate_string.is_empty()
}