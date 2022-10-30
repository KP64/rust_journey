// ! Evaluated at Compile time !!!
const fn get_first_n_numbers<const N: usize>() -> [usize; N] {
    let mut numbers = [0_usize; N];

    // * Note that for loops cant (yet) be used in const functions.
    // * So we have to go for a while loop.
    let mut i = 0;

    while i < N {
        numbers[i] = i + 1;
        i += 1;
    }

    numbers
}

// ! Evaluated at Compile time !!!
const fn get_five_numbers() -> [usize; 5] {
    let mut numbers = [0_usize; 5];

    // * Note that for loops cant (yet) be used in const functions.
    // * So we have to go for a while loop.
    let mut i = 0;

    while i < 5 {
        numbers[i] = i + 1;
        i += 1;
    }

    numbers
}
pub fn five_numbers() {
    println!("Five Numbers:");

    let numbers = get_five_numbers();
    println!("{:?}", numbers);

    // * TurboFish syntax
    let numbers = get_first_n_numbers::<11>();
    println!("TFS: {:?}", numbers);

    // * Explicit Type Syntax
    let numbers: [usize; 9] = get_first_n_numbers();
    println!("ETS: {:?}", numbers);
}
