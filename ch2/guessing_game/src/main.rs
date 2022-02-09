// needed libraries
use std::io;            // user input
use rand::Rng;          // random number generation
use std::cmp::Ordering; // comparing values

fn main() {
    println!("Guess the number!");

    // generates random number
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        // creates new mutable value to hold the guess
        let mut guess = String::new();

        io::stdin()
            // reads input from user and stores it in guess
            .read_line(&mut guess)
            // if a non "ok" result is received, this error is thrown
            .expect("Failed to read line");

        // shadow variable to re format our guess into the data type we want
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compares guess to our generated secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // breaks out of loop after a correct guess
            }
        }
        
    }
    
}
