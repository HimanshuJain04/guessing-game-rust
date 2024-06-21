use rand::Rng; // Import random number generator
use std::cmp::Ordering; // For comparing the guessed number and the secret number
use std::io; // For handling user input

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Guess the number between 1 and 100");

    // Generate number between 0-100
    let computer_guessed_number: u8 = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess:");

        // New variable for storing user input
        let mut user_input: String = String::new();

        // Read input from terminal
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input!!");

        // Parse user input
        let user_input: u8 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter valid number");
                continue;
            }
        };

        // Match the number
        match user_input.cmp(&computer_guessed_number) {
            Ordering::Less => println!("Guess higher"),
            Ordering::Greater => println!("Guess lower"),
            Ordering::Equal => {
                println!("You guessed it! The number was {}", computer_guessed_number);
                break;
            }
        }
    }
}
