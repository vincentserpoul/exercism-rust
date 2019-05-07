use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let _w: Vec<char> = sort_string(word.to_lowercase().as_ref());

    // filters all anagrams according to letter
    possible_anagrams
        .iter()
        .filter(|a| is_anagram(word, &_w, a))
        .cloned()
        .collect::<HashSet<&str>>()
}

fn is_anagram(word: &str, sorted_word: &[char], possible_anagram: &str) -> bool {
    // if words are not the same size
    if possible_anagram.chars().count() != sorted_word.len() {
        return false;
    }
    let possible_anagram_lc = possible_anagram.to_lowercase();
    // if words are the same
    if possible_anagram_lc == word.to_lowercase() {
        return false;
    }
    let sw = sort_string(&possible_anagram_lc);
    sw.iter()
        .enumerate()
        .find(|(i, c)| **c != sorted_word[*i])
        .is_none()
}

fn sort_string(s: &str) -> Vec<char> {
    let mut c = s.chars().collect::<Vec<char>>();
    c.sort();
    c
}

#[test]
fn test_is_anagram() {
    let dword = "adeipr";
    let dw: Vec<char> = vec!['a', 'd', 'e', 'i', 'p', 'r'];

    assert_eq!(is_anagram(dword, &dw, "d"), false);
    assert_eq!(is_anagram(dword, &dw, "diaper"), true);
    assert_eq!(is_anagram(dword, &dw, "Idaper"), true);
    assert_eq!(is_anagram(dword, &dw, "adeipr"), false);

    let oword = "Orchestra";
    let ow: Vec<char> = vec!['a', 'c', 'e', 'h', 'o', 'r', 'r', 's', 't'];

    assert_eq!(is_anagram(oword, &ow, "Carthorse"), true, "is orchestra");
}
