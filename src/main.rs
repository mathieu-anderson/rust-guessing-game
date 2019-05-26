// Prelude https://doc.rust-lang.org/std/prelude/index.html
// Import io from std
use std::io;
// Also possible
// use std::io::stdin;

use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Can you guess the number?");

    // Initialize a rng local to thread of execution and seeded by OS
    // Then generate u32 number in range (inclusive lower bound, exclusive upper bound)
    let number_to_guess = rand::thread_rng().gen_range(1, 101);

    // Infinite loop to prompt for more guesses
    loop {
        println!("Please input your guess :");

        // Create mutable variable bound to empty string
        let mut guess = String::new();

        // stdin()
        io::stdin()
            // Use mutable reference to guess string as argument to read_line
            .read_line(&mut guess)
            // Error handling :
            // if read_line returns Ok, expect holds and returns the returned Result value
            // if read_line returns Err, returns the string passed to expect
            .expect("Failed to read line");

        // Shadow previous value of guess and convert it to u32 number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input a number or it won't work 😵");
                continue;
            }
        };

        // Print guess string thorugh {} placeholder
        println!("You guessed: {}. {} is a good number 💯", guess, guess);

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("...but it's unfortunately too low 😔"),
            Ordering::Greater => println!("...but it's unfortunately too high 😔"),
            Ordering::Equal => {
                println!("... and it's the right answer 🧐");
                // Exits program when number is guessed
                break;
            }
        }
    }
}
