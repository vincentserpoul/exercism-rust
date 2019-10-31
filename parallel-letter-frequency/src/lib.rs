use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    input
        .iter()
        .map(|sentence| {
            sentence
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                })
        })
        .fold(HashMap::<char, usize>::new(), |mut acc, hm| {
            for (letter, count) in hm {
                *acc.entry(letter).or_insert(0) += count;
            }

            acc
        })
}
