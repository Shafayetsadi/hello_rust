use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=10); // ..= -> inclusive range
    let mut win: bool = false;

    println!("You have 3 chances to guess the number!");

    for _ in 1..=3 {
        println!("Enter your guess: ");

        let mut guess: String = String::new(); // mutable variable with new instance of String

        io::stdin()
            .read_line(&mut guess) // read and store in guess, &mut -> mutable reference
            .expect("Failed too read line"); // handling error
                                             /*
                                                     let guess: i32 = guess
                                                         .trim() // remove whitespace
                                                         .parse() // parse string to integer
                                                         .expect("Please type a number!"); // handling error
                                             */
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!")
            }
            Ordering::Equal => {
                println!("You got it!");
                win = true;
                break;
            }
            Ordering::Greater => {
                println!("Too big!")
            }
        }
    }

    if !win {
        println!("You lose! The secret number is: {}", secret_number);
    }
}
