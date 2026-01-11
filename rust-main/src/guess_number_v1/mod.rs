use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

pub fn run_engine() {
    let secret_number = rand::rng().random_range(1..=100);
    let mut tries = 10;
    let mut input = String::new();

    while tries > 0 {
        input.clear();
        print!("Enter a number: ");
        io::stdout().flush().expect("Failed");

        io::stdin()
                .read_line(&mut input)
                .expect("Whoops...something broke");


        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        tries -= 1;

         // Use match for cleaner comparison logic
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Whoops... higher"),
            Ordering::Greater => println!("Whoops... lower"),
            Ordering::Equal => {
                println!("You got it! -> {}", secret_number);
                return; // Exit game
            }
        }

    }

    println!("Game over! The number was: {}", secret_number);
}