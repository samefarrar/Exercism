use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a, 'b>(word: &str, possible_anagrams: &'b [&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter()
        .filter(|&&s| possible_anagram_operator(word, s))
        .cloned()
        .collect()
}

fn possible_anagram_operator(word: &str, possible_anagram: &str) -> bool {
    if word == possible_anagram {
        return false;
    }
    let lc_word = check_lowercase_and_convert(word);
    let lc_anagram = check_lowercase_and_convert(possible_anagram);
    let input_word = lc_word.as_str();
    let input_anagram = lc_anagram.as_str();
    if lc_word == input_anagram {
        return false
    }
    let is_anagram = check_anagram(input_word, input_anagram);
    return is_anagram
}

fn is_lowercase(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_lowercase() {
            return false
        }
    }
    true
}

fn check_lowercase_and_convert(s: &str) -> String {
    if is_lowercase(s) {
        return s.to_string();
    } else {
        return s.to_lowercase();
    }
}

fn check_anagram(word: &str, possible_anagram: &str) -> bool {
    if word.len() != possible_anagram.len() {
        return false
    }
    let letter_count_map1 = build_letter_count_dict(word);
    let letter_count_map2 = build_letter_count_dict(possible_anagram);
    for (key, value) in &letter_count_map1 {
        if letter_count_map2.get(key) != Some(value) {
            return false
        }
    }
    return true
}

fn build_letter_count_dict(word: &str) -> HashMap<char, u8> {
    let mut letter_count_map = HashMap::new();
    for c in word.chars() {
        *letter_count_map.entry(c).or_insert(0) += 1;
    }
    letter_count_map
}
