pub(crate) fn calculator() {
    println!("Calculator:");
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // ! "fn" is a pointer to a function
    fn calc_and_print(x: i32, y: i32, calculator: fn(i32, i32) -> i32) {
        let result = calculator(x, y);
        println!("{}", result);
    }

    calc_and_print(1, 2, add);
    calc_and_print(4, 5, |x, y| x + y);

    /*
     * mismatched types
     * expected fn pointer `fn(i32, i32) -> i32`
     * found closure
     */
    /*
     ! let z = 3;
     ! calc_and_print(1, 2, |x, y| x + y + z);
    */
}
