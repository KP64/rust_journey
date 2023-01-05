fn main() {
    println!("Hello, world!👋");

    let h = get_first_character("Hello, world!👋");
    println!("{:?}", h);
    let h = get_first_character_trimmed("              ⭐");
    println!("{:?}", h);

    // Built ins
    let h = "Hello, world!👋".chars().next();
    println!("{:?}", h);
    let h = ("              ⭐").chars().next();
    println!("{:?}", h);
}

/// getting first character from String even if it is a whitespace character
fn get_first_character(s: &str) -> Option<&str> {
    if s.is_empty() {
        return None;
    }

    let mut i = 0;
    loop {
        match s.get(..=i) {
            Some(x) => return Some(x),
            None => i += 1,
        }
    }
}

/// Trim the String and get the first character
fn get_first_character_trimmed(s: &str) -> Option<&str> {
    get_first_character(s.trim())
}
