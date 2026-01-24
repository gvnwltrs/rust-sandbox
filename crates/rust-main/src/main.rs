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

    println!("1. Updating a variable");
    updating_a_variable(); 

    println!("2. Setting a constant");
    set_a_constant();

    println!("3. Shadowing variables");
    performing_shadowing();

    println!("4. Writing to a buffer to format a string");
    let mut buf = String::new(); 
    let _ = write_fmt_to_buf(1, &mut buf);
    println!("{:?}\n\n", buf);

    println!("5. Using expressions");
    add_expressions(2, 2);

    println!("6. Using conditional expressions");
    conditional_expression(3, 4);
    println!("\n");

    println!("6. Using statements");
    println!("\n");

    println!("7. Using multiple conditionals");
    wrap_around_conditional(10);
    println!("\n");

    println!("8. Using if let");
    println!("result: {:?}\n", if_let(1));

    Ok(())
}


pub type EmptyString<'a> = &'a str;
#[allow(unused)]
const EMPTY_STR: EmptyString = "";