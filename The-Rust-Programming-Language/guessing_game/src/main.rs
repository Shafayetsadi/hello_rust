use std::cmp::Ordering;
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

    let guess: i32 = guess
        .trim() // remove whitespace
        .parse() // parse string to integer
        .expect("Please type a number!"); // handling error

    match guess.cmp(&secret_number) {
        Ordering::Less => { println!("Too small!") }
        Ordering::Equal => { println!("You got it!") }
        Ordering::Greater => { println!("Too big!") }
    }

    println!("The secret number is: {}", secret_number);
    println!("You guessed: {}", guess);

}
