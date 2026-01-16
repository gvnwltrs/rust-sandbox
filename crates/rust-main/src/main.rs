use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

#[allow(unused)]
use rust_main::{Assignments, borrow_checker, shadowing};

fn main() -> Result<(), Error> {
    // Start
    println!("Rust Main Starting...");

    // Shadowing example
    println!("{:#?}", shadowing());

    // Check 
    borrow_checker();

    // Use empty string convenience type
    println!("{}", EMPTY_STR);

    Ok(())
}


pub type EmptyString<'a> = &'a str;
const EMPTY_STR: EmptyString = "";