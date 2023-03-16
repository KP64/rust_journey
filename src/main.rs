#![warn(
    clippy::all,
    clippy::style,
    clippy::suspicious,
    clippy::nursery,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf
)]
use std::collections::HashMap;

fn main() {
    println!("Hello, world!👋");

    let h = get_first_character("Hello, world!👋");
    println!("{h:?}");
    let h = get_first_character_trimmed("              ⭐");
    println!("{h:?}");

    // Built ins
    let h = "Hello, world!👋".chars().next();
    println!("{h:?}");
    let h = ("              ⭐").chars().next();
    println!("{h:?}");
}

/// getting first character from String even if it is a whitespace character
fn get_first_character(s: &str) -> Option<&str> {
    if s.is_empty() {
        return None;
    }

    let mut i = 0;
    loop {
        match s.get(..=i) {
            Some(x) => break Some(x),
            None => i += 1,
        }
    }
}

/// Trim the String and get the first character
fn get_first_character_trimmed(s: &str) -> Option<&str> {
    get_first_character(s.trim())
}

#[must_use]
pub fn num_identical_pairs(nums: &[i32]) -> i32 {
    let mut hs = HashMap::<&i32, i32>::new();
    nums.iter()
        .map(|i| *hs.entry(i).and_modify(|f| *f += 1).or_insert(1) - 1)
        .sum()
}

#[must_use]
pub fn left_rigth_difference(nums: &[i32]) -> Vec<i32> {
    let mut answer = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        answer
            .push((nums.iter().take(i).sum::<i32>() - nums.iter().skip(i + 1).sum::<i32>()).abs());
    }
    answer
}
