pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        return None;
    }

    let mut primes = vec![2, 3];
    let mut next_prime: u32 = 4;

    while primes.len() < n as usize {
        let mut found_divider: bool = false;
        for p in &primes {
            if next_prime % p == 0 {
                found_divider = true;
                break;
            }
        }
        // meaning if we found no divider
        if !found_divider {
            primes.push(next_prime);
        }
        next_prime += 1
    }

    return Some(primes[(n - 1) as usize]);
}
