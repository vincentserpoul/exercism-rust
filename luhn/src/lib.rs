/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // remove spaces
    let clean_code = code.chars().filter(|c| *c != ' ').collect::<String>();

    if clean_code.len() <= 1 {
        return false;
    }
    if !clean_code.chars().all(char::is_numeric) {
        return false;
    }

    if clean_code
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let d = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                return d;
            }
            if (d * 2) > 9 {
                return (d * 2) - 9;
            }
            (d * 2)
        }).sum::<u32>()
        % 10
        != 0
    {
        return false;
    }

    true
}
