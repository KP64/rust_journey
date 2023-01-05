use rand::{thread_rng, Rng};

struct Password {
    length: usize,
}

impl Password {
    #[allow(unused)]
    fn new() -> Self {
        Self::with_length(10)
    }

    fn with_length(length: usize) -> Self {
        Self { length }
    }
}

impl IntoIterator for Password {
    type Item = String;

    type IntoIter = PasswordIterator;

    fn into_iter(self) -> Self::IntoIter {
        PasswordIterator {
            length: self.length,
        }
    }
}

struct PasswordIterator {
    length: usize,
}

impl Iterator for PasswordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::with_capacity(self.length);
        for _ in 0..self.length {
            result.push(thread_rng().gen_range('a'..='z'));
        }
        Some(result)
    }
}

pub(crate) fn self_impl() {
    println!("SelfImple:");

    let password_len = thread_rng().gen_range(0..=10);

    for p in Password::with_length(password_len).into_iter().take(3) {
        println!("The next password is {}", p);
    }
}
