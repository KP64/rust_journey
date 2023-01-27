#![feature(const_trait_impl, const_option)]

use std::{cmp::Ordering, convert::AsRef, time::SystemTime};

const ARRAY: [i32; 6] = [34, 45, 70, 200, 3535, 235235];
const KEY: i32 = 70;

fn main() {
    let sys_time = SystemTime::now();
    let answer = binary_search(ARRAY, KEY).unwrap();
    dbg!(sys_time.elapsed().unwrap().as_nanos());
    println!("The key \"{KEY}\" is at index: {answer}");

    let sys_time = SystemTime::now();
    const CONST_ANSWER: usize = const_binary_search(&ARRAY, &KEY).unwrap();
    dbg!(sys_time.elapsed().unwrap().as_nanos());
    println!("The key \"{KEY}\" is at index: {CONST_ANSWER:?}");
}

fn binary_search<T, V>(arr: V, key: T) -> Option<usize>
where
    T: Ord,
    V: AsRef<[T]>,
{
    let arr = arr.as_ref();
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(&key) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid,
        };
    }

    None
}

const fn const_binary_search<T>(arr: &[T], key: &T) -> Option<usize>
where
    T: ~const Ord,
{
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(key) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid,
        }
    }

    None
}
