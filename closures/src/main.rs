mod trait_types;
use trait_types::{fn_mut_trait, fn_once_trait, fn_trait};

// regular function
#[allow(unused)]
const fn add(x: i32, y: i32) -> i32 {
    x + y
}
#[allow(clippy::redundant_closure_call)]
fn main() {
    println!("Main:");

    // function as closure
    let _ = |x: i32, y: i32| -> i32 { x + y };

    // simplified closure because of singel expression
    let _ = |x: i32, y: i32| x + y;

    // Closure with inferred parameter types
    let f = |x, y| x + y;

    // * Inline closure including closure call
    println!("{}", (|x, y| x + y)(1, 2));

    let result = f(1, 2);
    println!("{result}");

    println!();
    fn_trait::fn_trait();
    println!();
    fn_mut_trait::fn_mut_trait();
    println!();
    fn_once_trait::fn_once_trait();
    println!();
}
