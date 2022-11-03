fn main() {
    println!("Hello, world!👋");

    let h = get_first_character("Hello, world!👋");
    println!("{}", h);
    let h = get_first_character_trimmed("              ⭐");
    println!("{}", h);
}

/// getting first character from String even if it is a whitespace character
fn get_first_character(s: &str) -> &str {
    if s.is_empty() {
        return "";
    }

    let mut i = 0;
    loop {
        return match s.get(..=i) {
            Some(x) => x,
            None => {
                i += 1;
                continue;
            }
        };
    }
}

fn get_first_character_trimmed(s: &str) -> &str {
    let s = s.trim();
    if s.is_empty() {
        return "";
    }

    let mut i = 0;
    loop {
        return match s.get(..=i) {
            Some(x) => x,
            None => {
                i += 1;
                continue;
            }
        };
    }
}
