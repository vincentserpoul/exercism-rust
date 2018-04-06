pub fn raindrops(n: usize) -> String {
    let mut s = "".to_string();
    if n % 3 == 0 {
        s.push_str("Pling");
    }
    if n % 5 == 0 {
        s.push_str("Plang");
    }
    if n % 7 == 0 {
        s.push_str("Plong");
    }
    if s.chars().count() == 0 {
        return n.to_string();
    }

    return s;
}
