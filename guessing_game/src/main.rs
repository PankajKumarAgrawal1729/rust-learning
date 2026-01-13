// Import required modules for the guessing game
use rand::Rng;                  // For random number generation
use std::cmp::Ordering;         // For comparing values (Less, Greater, Equal)
use std::io;                    // For reading user input

fn main() {
    // Display welcome message to the player
    println!("Welcome to Number Guessing Game!!");

    // Generate a random secret number between 1 and 100 (inclusive)
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // Debug line (commented out): Uncomment to see the secret number
    // println!("The secret number is: {secret_number}");

    // Main game loop - continues until the player guesses correctly
    loop {
        println!("Please input your Guess");

        // Create a mutable String to store user input
        let mut guess = String::new();

        // Read a line of input from standard input (user's keyboard)
        // expect() will panic if reading fails
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Parse the input string into a u32 (unsigned 32-bit integer)
        // If parsing fails, display an error message and skip to next iteration
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,                              // Successfully parsed a number
            Err(_) => {                                  // Parsing failed (invalid input)
                println!("Please enter a valid number!");
                continue;                               // Skip to next loop iteration
            },
        };

        // Display the number that was guessed
        println!("Guessed Number: {guess}");

        // Compare the guess with the secret number and provide feedback
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),    // Guess is smaller than secret
            Ordering::Greater => println!("Too Big"),   // Guess is larger than secret
            Ordering::Equal => {                        // Guess matches the secret number
                println!("You Win!");
                break;                                  // Exit the loop - game is won
            }
        }
    }
}
