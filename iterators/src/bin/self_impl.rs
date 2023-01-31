use rand::{thread_rng, Rng};

struct Password {
    length: usize,
}

impl Password {
    const fn new() -> Self {
        Self::with_length(10)
    }

    const fn with_length(length: usize) -> Self {
        Self { length }
    }
}

impl IntoIterator for Password {
    type Item = String;

    type IntoIter = PasswordIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.length)
    }
}

struct PasswordIterator {
    length: usize,
}
impl PasswordIterator {
    const fn new(length: usize) -> Self {
        Self { length }
    }
}

impl Iterator for PasswordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            (0..self.length)
                .map(|_| thread_rng().gen_range('a'..='z'))
                .collect(),
        )
    }
}

fn main() {
    println!("SelfImple:");

    let password_len = thread_rng().gen_range(1..=10);
    Password::with_length(password_len)
        .into_iter()
        .take(3)
        .for_each(|p| println!("The next password is {p}"));

    Password::new()
        .into_iter()
        .take(3)
        .for_each(|p| println!("The next password is {p}"));
}
