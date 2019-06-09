use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // filters all anagrams according to letter
    possible_anagrams
        .iter()
        .filter(|a| is_anagram(word, a))
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
        .find(|(i, ch)| **ch != word_sorted_chars[*i])
        .is_none()
}

fn sort_string(s: &str) -> Vec<char> {
    let mut word_sorted_chars = s.chars().collect::<Vec<char>>();
    word_sorted_chars.sort();
    word_sorted_chars
}

#[test]
fn test_is_anagram() {
    let dword = "adeipr";

    assert_eq!(is_anagram(dword, "d"), false);
    assert_eq!(is_anagram(dword, "diaper"), true);
    assert_eq!(is_anagram(dword, "Idaper"), true);
    assert_eq!(is_anagram(dword, "adeipr"), false);

    let oword = "Orchestra";

    assert_eq!(is_anagram(oword, "Carthorse"), true, "is orchestra");
}
