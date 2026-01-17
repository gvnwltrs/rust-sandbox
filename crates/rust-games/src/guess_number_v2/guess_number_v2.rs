use std::cmp::Ordering;
use std::io;
use std::io::Write;

pub struct Game {
    secret: u32,
    tries_left: u32,
    guesses: Vec<u32>,
}

pub enum StepResult {
    Continue,
    Win,
    Lose,
}

impl Game {
    pub fn setup(secret: u32, tries: u32) -> Self {
        Self {
            secret: secret,
            tries_left: tries,
            guesses: Vec::<u32>::new(),
        }
    }

    pub fn request_guess(&mut self) -> io::Result<Option<u32>> {
        let mut input = String::new();

        print!("Enter a number: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(guess) => Ok(Some(guess)),
            Err(_) => {
                println!("Please enter a valid number!");
                Ok(None)
            }
        }
    }

    pub fn step(&mut self, guess: u32) -> StepResult {
        self.guesses.push(guess);

        // Check win BEFORE try exhaustion logic
        match guess.cmp(&self.secret) {
            Ordering::Equal => return StepResult::Win,
            Ordering::Less => println!("Higher"), 
            Ordering::Greater => println!("Lower"), 
        }

        // Consume a try for a valid (but incorrect) guess
        if self.tries_left > 0 {
            self.tries_left -= 1;
        }

        if self.tries_left == 0 {
            StepResult::Lose
        } else {
            StepResult::Continue
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        while self.tries_left > 0 {
            let guess = match self.request_guess()? {
                Some(g) => g,
                None => continue, // invalid input, no try consumed
            };

            match self.step(guess) {
                StepResult::Continue => {}
                StepResult::Win => { 
                    println!("You win! the number was {}", self.secret); 
                    return Ok(()); 
                }
                StepResult::Lose => { 
                    println!("Game over! The number was {}", self.secret); 
                    return Ok(());
                }
            }
        }

        // If tries starts at 0
        println!("Game over! The number was {}", self.secret);
        Ok(())
    }
}

// fn main() {
//     let mut game = Game {
//         secret: 42,
//         tries_left: 5,
//     };

//     let guesses = [10, 50, 40, 42];

//     for guess in guesses {
//         match game.step(guess) {
//             StepResult::Continue => {}
//             StepResult::Win => {
//                 println!("You win!");
//                 return;
//             }
//             StepResult::Lose => {
//                 println!("Game over!");
//                 return;
//             }
//         }
//     }
// }