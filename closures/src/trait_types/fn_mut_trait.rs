pub fn fn_mut_trait() {
    println!("FnMut Trait:");
    let mut result = 0;

    // * Closure with mutable borrow
    let mut calc_result = |x, y| result = x + y;
    calc_result(1, 2);
    println!("{result}");

    // * Store closure in "FnMut" variable before calling it
    let mut result_calculator: Box<dyn FnMut(i32, i32)> = Box::new(|x, y| result = x + y);
    result_calculator(100, 124);
    drop(result_calculator);
    println!("{result}");
}
