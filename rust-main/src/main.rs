use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

// use rust_main::repeat_until::repeat_until;
// use rust_main::guess_number_v1::run_engine;
// use rust_main::guess_number_v2::Game;
// use rust_main::counters::count_to_5;
use rust_main::commands::{ run_ls, sys_cmd };
use rust_main::network::start_scan;


fn main() -> Result<(), Error> {
    println!("Rust Main Starting...");
    
    // simple count
    // count_to_5();

    // game v1
    // run_engine();

    // game v2
    // let mut new_game = Game::setup(42, 5);
    // new_game.start();

    // run an os system level "ls" call 
    // let cmd_ls = run_ls();
    // println!("command executed: {}", cmd_ls.is_ok());

    // let cmd = sys_cmd("pwd");
    // println!("command executed: {}", cmd.is_ok());
    // match cmd {
    //     Ok(x) => println!("result: {:#?}", cmd.unwrap_or(x)),
    //     Err(_) => todo!(),
    // }

    // let top = sys_cmd("date");

    // network scan 
    start_scan();

    Ok(())
}
