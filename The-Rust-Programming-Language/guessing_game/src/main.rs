use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng()
        .gen_range(1..=10); // ..= -> inclusive range

    println!("Enter your guess: ");

    let mut guess: String = String::new(); // mutable variable with new instance of String

    io::stdin()
        .read_line(&mut guess) // read and store in guess, &mut -> mutable reference
        .expect("Failed too read line"); // handling error

    println!("The secret number is: {}", secret_number);
    println!("You guessed: {}", guess);

}
