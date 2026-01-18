#[allow(unused)]
use std::io::Error;
#[allow(unused)]
use rust_games::guess_number_v1;
#[allow(unused)]
use rust_games::guess_number_v2::{Game, StepResult};

// fn main() -> Result<(), Box<dyn std::error::Error>>{
fn main() -> std::io::Result<()>{
    println!("Guess the Number Game Starting.");

    // Using guess number version 2

    // Mannual setup
    // let mut game = Game {
    //     secret: 42,
    //     tries_left: 5,
    //     guesses: Vec::<u32>::new(),
    // };


    // // Automated approach
    // let guesses = [10, 50, 40, 42];

    // for guess in guesses {
    //     match game.step(guess) {
    //         StepResult::Continue => {}
    //         StepResult::Win => {
    //             println!("You win!");
    //             return Ok(());
    //         }
    //         StepResult::Lose => {
    //             println!("Game over!");
    //             return Ok(());
    //         }
    //     }
    // }

    // Using API 
    let mut game = Game::setup(42, 5); 
    let _result = game.start();
    Ok(())

}
