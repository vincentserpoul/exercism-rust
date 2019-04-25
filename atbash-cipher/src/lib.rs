use std::iter::FromIterator;

const ALPH: &str = "abcdefghijklmnopqrstuvwxyz";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
        plain.to_lowercase()
                .chars()
                .filter(char::is_ascii_alphanumeric)
                .map(|c| match c {
                        c if c.is_numeric() => c,
                        _ => ALPH.chars().rev().nth(ALPH.find(c).unwrap()).unwrap(),
                })
                .collect::<Vec<char>>()
                .chunks(5)
                .map(String::from_iter)
                .collect::<Vec<String>>()
                .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
        cipher.chars()
                .filter(char::is_ascii_alphanumeric)
                .map(|c| match c {
                        c if c.is_numeric() => c,
                        _ => ALPH.chars().rev().nth(ALPH.find(c).unwrap()).unwrap(),
                })
                .collect::<String>()
}
