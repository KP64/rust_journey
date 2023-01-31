use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    fmt::Display,
    ops::Range,
    thread,
    time::{Duration, Instant},
};

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
const X: u128 = 10;
const ITERATION_RANGE: Range<u128> = 0..X;
fn some_function_to_time() {
    thread::sleep(Duration::from_secs(1));
}
fn main() {
    // * For loop
    let now = Instant::now();
    for _ in ITERATION_RANGE {
        some_function_to_time();
    }
    let elapsed = now.elapsed();
    print_time_taken("For-Loop", elapsed.as_nanos(), "nano-secs");
    print_time_taken("For-Loop", elapsed.as_secs(), "secs");
    println!();

    // * Iterator
    let now = Instant::now();
    ITERATION_RANGE.for_each(|_| some_function_to_time());
    let elapsed = now.elapsed();
    print_time_taken("Iter", elapsed.as_nanos(), "nano-secs");
    print_time_taken("Iter", elapsed.as_secs(), "secs");
    println!();

    // * Parallel Iterator
    // ? Fastest By a WIDE MARGIN
    let now = Instant::now();
    ITERATION_RANGE
        .into_par_iter()
        .for_each(|_| some_function_to_time());
    let elapsed = now.elapsed();
    print_time_taken("ParIter", elapsed.as_nanos(), "nano-secs");
    print_time_taken("ParIter", elapsed.as_secs(), "secs");
    println!();
}

fn print_time_taken(iter_method: &str, time_taken: impl Display, time_unit: &str) {
    println!("{iter_method} Time: {time_taken} {time_unit}");
}
