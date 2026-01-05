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
use rust_main::functional::square;
use rust_main::math::add_two;

#[allow(unused)]
enum SelectLib {
    GuessNumberV1,
    GuessNumberV2,
    Counters,
    Commands,
    Network,
    Functional,
    Math,
}

fn main() -> Result<(), Error> {
    println!("Rust Main Starting...");

    let selection = SelectLib::Math;

    match selection {
        SelectLib::GuessNumberV1 => {
            // game v1
            run_engine();
        }
        SelectLib::GuessNumberV2 => {
            // game v2
            let mut new_game = Game::setup(42, 5);
            let _ =new_game.start();
        }
        SelectLib::Counters => {
            // simple count
            count_to_5();
        }
        SelectLib::Commands => {
            // run an os system level "ls" call 
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
            // network scan 
            start_scan();
        }
        SelectLib::Functional => {
            // map a squared result to 0 through 10 
            let product= square::<i32>(2);
            println!("squared result: {}", product);

            // use closure to return mapped execution
            let x: Vec<_> = (0..10).map(|x| x*x).collect();
            println!("x: {:#?}", x);
        }
        SelectLib::Math => {
            let result = add_two(1, 1);
            println!("result: {:#?}", result);
        }
    }

    Ok(())
}
