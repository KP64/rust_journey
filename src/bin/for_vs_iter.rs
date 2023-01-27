use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{ops::Range, time::SystemTime};

/*
! Performance:
* Debug:
* ParIter >= For >= Iter
*
* Release:
* Iter >= For >= ParIter
*/
// ? Explanation of performance
// * Debug:
// ? In Debug mode the ParIter is the Fastest since it will be operating other tasks
// ? While some tasks are being checked on (Debug checks).
// * Release:
// ? The Debug checks are disabled, why the main Benefit of ParIter is eliminated.
// ? Note: this test is superficial since the function does not do anything IO-Bound => Real case scenario is different.
// ? The compiler will most likely be optimize away all loops except for ParIter since it's a different Crate
const X: u128 = 1_000_000_000;
const ITERATION_RANGE: Range<u128> = 0..X;
const fn some_function_to_time() {}
fn main() {
    // * For loop
    let before = SystemTime::now();
    for _ in ITERATION_RANGE {
        some_function_to_time();
    }
    let avg = before.elapsed().unwrap().as_nanos();
    print_time_taken("For-Loop", avg, "nano-secs");
    print_time_taken("For-Loop", avg / X, "secs");
    println!();

    // * Iterator
    // ? Slower than For loop
    let before = SystemTime::now();
    ITERATION_RANGE.for_each(|_| some_function_to_time());
    let avg = before.elapsed().unwrap().as_nanos();
    print_time_taken("Iter", avg, "nano-secs");
    print_time_taken("Iter", avg / X, "secs");
    println!();

    // * Parallel Iterator
    // ? Fastest
    // ! By a WIDE MARGIN (only in Debug mode)
    let before = SystemTime::now();
    ITERATION_RANGE
        .into_par_iter()
        .for_each(|_| some_function_to_time());
    let avg = before.elapsed().unwrap().as_nanos();
    print_time_taken("ParIter", avg, "nano-secs");
    print_time_taken("ParIter", avg / X, "secs");
    println!();
}

fn print_time_taken(iter_method: &str, time_taken: u128, time_unit: &str) {
    println!("{iter_method} Time: {time_taken} {time_unit}");
}
