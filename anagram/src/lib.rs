use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // filters all anagrams according to letter
    possible_anagrams
        .iter()
        .filter(|possible_anagram| is_anagram(word, possible_anagram))
        .cloned()
        .collect::<HashSet<&str>>()
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    let word_sorted_chars = sort_string(word.to_lowercase().as_ref());

    // if words are not the same size
    if possible_anagram.chars().count() != word_sorted_chars.len() {
        return false;
    }
    let possible_anagram_low = possible_anagram.to_lowercase();
    // if words are the same
    if possible_anagram_low == word.to_lowercase() {
        return false;
    }
    sort_string(&possible_anagram_low)
        .iter()
        .enumerate()
        .find(|(idx, ch)| **ch != word_sorted_chars[*idx])
        .is_none()
}

fn sort_string(s: &str) -> Vec<char> {
    let mut word_sorted_chars = s.chars().collect::<Vec<char>>();
    word_sorted_chars.sort();
    word_sorted_chars
}

#[test]
fn test_is_anagram() {
    let word_test_1 = "adeipr";

    assert_eq!(is_anagram(word_test_1, "d"), false);
    assert_eq!(is_anagram(word_test_1, "diaper"), true);
    assert_eq!(is_anagram(word_test_1, "Idaper"), true);
    assert_eq!(is_anagram(word_test_1, "adeipr"), false);

    let word_test_2 = "Orchestra";

    assert_eq!(is_anagram(word_test_2, "Carthorse"), true, "is orchestra");
}
