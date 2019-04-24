const ALPH: &str = "abcdefghijklmnopqrstuvwxyz";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
        plain.to_lowercase()
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<Vec<char>>()
                .chunks(5)
                .map(|w| {
                        w.iter().filter_map(|c| match c {
                                c if c.is_numeric() => Some(*c),
                                _ => ALPH.chars().rev().nth(ALPH.find(*c).unwrap()),
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
        cipher.chars()
                .filter(|c| c.is_alphanumeric())
                .filter_map(|c| {
                        if c.is_numeric() {
                                return Some(c);
                        }
                        match ALPH.find(c) {
                                None => None,
                                Some(idx) => ALPH.chars().rev().nth(idx),
                        }
                })
                .collect::<String>()
}
