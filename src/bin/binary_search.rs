use std::cmp::Ordering;

fn main() {
    let arr = Vec::from_iter([34, 45, 70, 200, 3535, 235235]);
    //let arr = [34, 45, 70, 200, 3535, 235235];
    let key = 70;

    let answer = binary_search(arr, key).unwrap();
    println!("The key \"{}\" is at index: {}", key, answer);
}

fn binary_search<T, V>(arr: V, key: T) -> Option<usize>
where
    T: Ord,
    V: AsRef<[T]>,
{
    let arr: &[T] = arr.as_ref();
    let mut low: usize = 0;
    let mut high: usize = arr.len();

    while low < high {
        let mid: usize = low + (high - low) / 2;
        match arr[mid].cmp(&key) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid,
        };
    }

    None
}
