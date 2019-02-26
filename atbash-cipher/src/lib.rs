const ALPH: &str = "abcdefghijklmnopqrstuvwxyz";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
        plain.to_lowercase()
                .chars()
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
                .enumerate()
                .fold(String::from(""), |mut s, (i, c)| {
                        if i != 0 && (i as i32) % 5 == 0 {
                                s.push(' ');
                        }
                        s.push(c);
                        s
                })
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
