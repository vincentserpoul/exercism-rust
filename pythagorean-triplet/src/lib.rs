pub fn find() -> Option<u32> {
    for a in 0..500 {
        for b in 0..500 {
            let c: u32 = 1000 - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}
