//Import library for accepting input and output
use std::io;
//Import library for generating random numbers
use rand::Rng;

// Import cmp library for comparing our data
use std::cmp::Ordering;

//Import color library for results
use colored::*;

fn main() {
    println!("Guess the number!");

    // Create a random number generator using Rand
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Looping through until we find correct number
    loop {
        println!("Please enter a number to guess");

        // Declare variable guess to store users input and it will be mutable
        let mut guess = String::new();

        //Accept input string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        //Converting guess to integer for comparison
        let guess: u32 = match guess.trim().parse() {
            // Error handling to prevent crash if not a number is provided
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("You guessed: {}", guess);

        println!("Secret Number is : {}", secret_number);

        // Using Match to determine whether the secret number matches the guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Greater!!".red()),
            //Breaking loop on Correct solution
            Ordering::Equal => {
                println!("{}", "Woohooooooo correct answer!!!".green());
                break;
            }
        }
    }
}

    