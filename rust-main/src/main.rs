use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

#[allow(unused_imports)]
use rust_main::guess_number_v1::run_engine;
use rust_main::guess_number_v2::Game;
use rust_main::counters::count_to_5;
use rust_main::commands::{ run_ls, sys_cmd };
use rust_main::network::start_scan;
use rust_main::math::*;
use rust_main::brain_teasers::*;

#[allow(unused)]
enum SelectLib {
    GuessNumberV1,
    GuessNumberV2,
    Counters,
    Commands,
    Network,
    Math,
    BrainTeasers,
}
use SelectLib::*;

fn main() -> Result<(), Error> {
    println!("Rust Main Starting...");

    let selection = Math;

    match selection {
        SelectLib::GuessNumberV1 => {
            // Game v1
            run_engine();
        }
        SelectLib::GuessNumberV2 => {
            // Game v2
            let mut new_game = Game::setup(42, 5);
            let _ =new_game.start();
        }
        SelectLib::Counters => {
            // Simple count
            count_to_5();
        }
        SelectLib::Commands => {
            // Run an os system level "ls" call 
            let cmd_ls = run_ls();
            println!("command executed: {}", cmd_ls.is_ok());

            let cmd = sys_cmd("pwd");
            println!("command executed: {}", cmd.is_ok());
            match cmd {
                Ok(x) => println!("result: {:#?}", cmd.unwrap_or(x)),
                Err(_) => todo!(),
            }

            let date = sys_cmd("date");
            println!("date executed: {}", date.is_ok());
        }
        SelectLib::Network => {
            // Network scan 
            start_scan();
        }
        SelectLib::Math => {
            let result = add_two(1, 1);
            println!("result: {:#?}", result);

            let mut result: i32 = 0;
            let result = add_some(2, 2, &mut result);
            println!("result: {:#?}", result);
        }
        SelectLib::BrainTeasers => {
            three_and_a_bit();
            non_standard_input(Select::GoodVersion);
        }
    }

    Ok(())
}
