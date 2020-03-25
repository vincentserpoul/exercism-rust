use std::collections::HashMap;
extern crate rayon;
use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap();

    pool.install(|| {
        input
            .par_iter()
            .map(|sentence| {
                sentence
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .fold(HashMap::<char, usize>::new(), |mut hm, ch| {
                        *hm.entry(ch).or_insert(0) += 1;
                        hm
                    })
            })
            .reduce(HashMap::<char, usize>::new, |mut hm, sentence_hm| {
                for (letter, count) in sentence_hm {
                    *hm.entry(letter).or_insert(0) += count;
                }
                hm
            })
    })
}
