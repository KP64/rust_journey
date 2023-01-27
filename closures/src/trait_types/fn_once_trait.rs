pub(crate) fn fn_once_trait() {
    println!("FnOnce Trait:");

    // * Closure consuming Iterator
    let numbers_iter = vec![1, 2, 3, 4, 5, 6].into_iter();
    let sum_calculator = move || numbers_iter.sum();
    let result: i32 = sum_calculator();
    println!("{result}");

    // * Store closure in "FnOnce" variable before calling it
    let numbers_iter = vec![18, 2345, 20, 2].into_iter();
    let sum_calculator: Box<dyn FnOnce() -> i32> = Box::new(move || numbers_iter.sum());
    let result = sum_calculator();
    println!("{result}");
}
