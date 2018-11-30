/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let c = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .collect::<Vec<char>>();

    if c.len() <= 1 {
        return false;
    }

    match c
        .iter()
        .enumerate()
        .try_fold(0, |acc, (i, c)| match c.to_digit(10) {
            None => Err("not a number"),
            Some(d) => Ok(add_digit(acc, i, d)),
        }) {
        Err(_) => return false,
        Ok(sum) => (sum % 10) == 0,
    }
}

// add_digit add a digit d to the accumulator acc, according to index i
fn add_digit(acc: u32, i: usize, d: u32) -> u32 {
    if i % 2 == 0 {
        return acc + d;
    }
    if (d * 2) > 9 {
        return acc + (d * 2) - 9;
    }
    acc + d * 2
}
