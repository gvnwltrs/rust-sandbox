use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

#[allow(unused)]
use rust_main::*;

fn main() -> Result<(), Error> {
    // Start
    println!("Rust Main Starting...\n");

    // Rust the Programming Language
    println!("Running through Rust the Programming Language concepts.\n");
    updating_a_variable(); 
    set_a_constant();
    performing_shadowing();

    Ok(())
}


pub type EmptyString<'a> = &'a str;
#[allow(unused)]
const EMPTY_STR: EmptyString = "";