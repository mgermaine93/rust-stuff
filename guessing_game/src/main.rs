// run this file via "cargo run" from within the root project directory

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!  (It's between 1 and 100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your guess:");

        // variables are mutable by default.
        // adding "mut" makes them mutable
        let mut guess = String::new();

        io::stdin()
            // "&..." indicates a "reference".
            // this allows multiple parts of my code to access one piece of data without needing to copy that data into memory multiple times.
            // references are also immutable by default.
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        // uses "shadowing" to convert the earlier value of "guess" from one value to another.
        // eliminates whitespace and/or newline/carriage return stuff.
        let guess: u32 = match guess
            .trim()
            .parse() {
                // "parse()" returns either "Ok" or "Err", so we handle both of those below.
                Ok(num) => num,
                // the "_" catches all errors, so it matches all "Err" values.
                Err(_) => continue,
            };

        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
    
}
