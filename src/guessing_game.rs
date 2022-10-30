use rand::Rng;
use std::{cmp::Ordering, io, ops::RangeInclusive};

const POSSIBLE_NUMS: RangeInclusive<i32> = 0..=20;
fn main() {
    let secret_number = rand::thread_rng().gen_range(POSSIBLE_NUMS);

    let mut remaining_tries = 6;

    while remaining_tries > 0 {
        println!("Please input your guess: "); // * print!("..."); doesnt work?

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => {
                if !(POSSIBLE_NUMS).contains(&num) {
                    println!("Your number is out of range.");
                    println!("The range is from {:?}\n", POSSIBLE_NUMS);
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please Enter a Number!\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The answer is too low!"),
            Ordering::Greater => println!("The answer is too high!"),
            Ordering::Equal => {
                println!("You have found the secret number!");
                println!("The answer is {}", secret_number);
                break;
            }
        };

        remaining_tries -= 1;
        if remaining_tries > 0 {
            println!(
                "You have only got {} {} left!\n",
                remaining_tries,
                if remaining_tries == 1 { "try" } else { "tries" }
            );
        } else {
            println!("You have lost.");
            println!("Good luck next Time! 💖");
        }
    }
}
