use std::time::SystemTime;

fn some_function_to_time() {}

/*
! Performance:
* Debug:
* While >= For >= Iter
*
* Release:
* While == For == Iter
*/
fn main() {
    let x = 500000000;

    let before = SystemTime::now();
    for _ in 0..x {
        some_function_to_time();
    }

    let after = SystemTime::now();
    let avg = after.duration_since(before).unwrap().as_secs();
    println!("Time: {:?} secs", avg);

    let before = SystemTime::now();
    // * Basically 0
    let _s = (0..x).map(|_| some_function_to_time());
    // * Slower
    let _s = (0..x).map(|_| some_function_to_time()).collect::<Vec<_>>();

    let after = SystemTime::now();
    let avg = after.duration_since(before).unwrap().as_secs();
    println!("Time: {:?} secs", avg);
}
