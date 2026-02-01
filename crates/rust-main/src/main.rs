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
    println!("Result: {:?}\n", if_let(1));

    println!("9. Using conditional loop");
    println!("Result: {:?}\n", conditional_loop(5));

    println!("10. Where do literal strings go?");
    let heap_string = String::from("heap_string");
    println!("Result: {:?}, Input: {:?}", where_does_this_string_live(&heap_string), heap_string);
    let stack_string = "";
    println!("Result: {:?}, Input: {:?}", where_does_this_string_live(stack_string), stack_string);
    let literal_str = "literal_str";
    println!("Result: {:?}, Input: {:?}\n", where_does_this_string_live(literal_str), literal_str);

    println!("11. Examples of ownership in functions");
    let mut s = String::from("this string");
    println!("We create a String: {:?}", s);
    let moved_s = std::mem::take(&mut s);
    println!("Now we pass the string to function not as a reference or borrow.");
    takes_ownership(moved_s);
    println!("Let's look at the string again: {:?}\n", s);

    let mut x = 5;
    println!("x is: {:?}", x);
    println!("Now we copy x to y.");
    let mut y = makes_copy(x);
    println!("y is: {:?}", y);
    x = 10; 
    println!("We modify x: {:?}", x); 
    println!("y is still: {:?}", y);
    y = 20;
    println!("We modify y: {:?}", y);
    println!("Now we have x:{:?}, y:{:?}\n", x,y);

    println!("12. Giving ownership from a function");
    let give_me = gives_ownership();
    println!("We call 'gives_ownership' and it gives us a string: {:?}\n", give_me);

    println!("13. Mutable references"); 
    let mut my_string = String::from("Before we had this...");
    println!("Original: {:?}", my_string);
    mutate_reference(&mut my_string);
    println!("\
        We can modify a reference so long as we claim that's \
        what we are doing with the thing we borrow: \
        \n{:?}\n", my_string
    );

    println!("14. Dangling references (TBD)...\n");

    println!("15. String slices");
    let my_words = String::from("This is it");
    println!("Original string: {:?}", my_words);
    let slice = first_word_slice(&my_words);
    println!("Slice: {:?}\n", slice);
    let another_slice = &my_words[0..7];
    println!("Another slice: {:?}\n", another_slice);

    println!("16. Handling structs");
    let istruct = init_struct();
    println!("Initalizing a struct and returning: {:#?}\n", istruct);
    let tstruct = init_tup_struct(42, 43);
    println!("Initalizing a tuple struct and returning: {:#?}\n", tstruct);

    println!("17. Using methods");
    let mut data = MyData::new(); // single owner
    println!("Initialized to: {:#?}", data);
    using_data_method(&mut data); 
    println!("After borrowing and modifying: {:#?}\n", data);
    println!(
        "The init was an \"Associated Function\",\
         while the function that called it was a free function\n\
    ");
    println!("Now calling method to reset.");
    data.reset();
    println!("Result for reset: {:#?}\n", data);

    println!("18. Using enums");
    let mut new_vault = Vault::Name(String::from("Old"));
    println!("Old vault name: {:#?}", new_vault);
    update_vault(&mut new_vault);
    println!("Updated vault name: {:#?}\n", new_vault);

    Ok(())
}


pub type EmptyString<'a> = &'a str;
#[allow(unused)]
const EMPTY_STR: EmptyString = "";