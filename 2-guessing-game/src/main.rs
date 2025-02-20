use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Infinitely loop
    loop {
        println!("Please input your guess");

        let mut guess = String::new();  // In rust variables default to be immutable

        // Read standard input and cast to int
        io::stdin().read_line(&mut guess).expect("Failed to read line!");  // References are also immutable by default
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  // Move to next iteration
        };

        println!("You guessed {}", guess);

        // Switch block
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}