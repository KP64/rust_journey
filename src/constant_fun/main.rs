mod animals;
mod buf_concat;
mod customers;
mod five_numbers;
mod german_goodbye;
mod last_number;
fn main() {
    last_number::last_number();
    println!();
    german_goodbye::german_goodbye();
    println!();
    five_numbers::five_numbers();
    println!();
    buf_concat::buf_concat();
    println!();
    customers::customers();
    println!();
    animals::animals();
    println!();
}
