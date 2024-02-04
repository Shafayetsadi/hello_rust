use std::io;

fn main() {
    println!("Hello, rust!");

    println!("Guess the number!");
    println!("Enter your guess: ");

    let mut guess = String::new(); // mutable variable with new instance of String

    io::stdin()
        .read_line(&mut guess) // read and store in guess, &mut -> mutable reference
        .expect("Failed too read line"); // handling error

    println!("You guessed: {}", guess);
}
