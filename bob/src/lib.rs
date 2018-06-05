pub fn reply(_message: &str) -> &str {
    let m = _message.trim();

    if m.is_empty() {
        return "Fine. Be that way!";
    }

    // uppercase and not all numeric
    if m.chars().any(char::is_alphabetic) && !m.chars().any(char::is_lowercase) {
        if m.ends_with("?") {
            return "Calm down, I know what I'm doing!";
        }
        return "Whoa, chill out!";
    }

    if m.ends_with("?") {
        return "Sure.";
    }

    "Whatever."
}
