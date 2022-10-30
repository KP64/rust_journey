use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..=100);

    let mut remaining_tries = 6;

    while remaining_tries > 0 {
        println!("Please input your guess: "); // * print!("..."); doesnt work?

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => {
                if !(0..=100).contains(&num) {
                    println!("Your number is out of range.");
                    println!("The range is from 0 to 100\n");
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
