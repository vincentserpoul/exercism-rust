use std::collections::HashMap;
<<<<<<< Updated upstream
<<<<<<< HEAD
=======
use std::thread;
use std::sync::mpsc;
>>>>>>> Stashed changes

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool = ThreadPool::new(worker_count);
    input
        .iter()
        .for_each(|sentence| {
             pool::execute(move || {
            sentence
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                })
             })
        });

        .fold(HashMap::<char, usize>::new(), |mut acc, hm| {
            for (letter, count) in hm {
                *acc.entry(letter).or_insert(0) += count;
            }

            acc
        })
=======
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
>>>>>>> 9cae8ad76f68a43730c1e48a0bfa775f4ea06bd1
}
