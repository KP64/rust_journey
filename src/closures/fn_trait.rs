pub fn fn_trait() {
    println!("CalculatorV2:");

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    // ! Fn is a trait allowing to take ownership of
    // ! variables outside of the closure Scope
    // * 1. Approach needs a lifetime:
    fn calc_and_print(x: i32, y: i32, calculator: Box<dyn Fn(i32, i32) -> i32 + '_>) {
        let result = calculator(x, y);
        println!("lifetimed: {}", result);
    }
    calc_and_print(1, 2, Box::new(add));
    calc_and_print(4, 29, Box::new(|x, y| x + y));

    // * Now we can also pass a closure with
    // * capturing to calc_and_print
    let z = 3;
    calc_and_print(1, 2, Box::new(|x, y| x + y + z));

    // * 2. Approach needs a reference:
    fn calc_and_print_borrowed(x: i32, y: i32, calculator: Box<&dyn Fn(i32, i32) -> i32>) {
        let result = calculator(x, y);
        println!("borrowed : {}", result);
    }
    calc_and_print_borrowed(1, 25, Box::new(&add));
    calc_and_print_borrowed(2, 29, Box::new(&|x, y| x + y));

    // * Now we can also pass a closure with
    // * capturing to calc_and_print
    let z = 3;
    calc_and_print_borrowed(188, 27, Box::new(&|x, y| x + y + z));
}
