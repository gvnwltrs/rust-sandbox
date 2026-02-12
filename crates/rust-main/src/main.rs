use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

#[allow(unused)]
use rust_main::*;


fn main() -> Result<(), Error> {
    // Message helpers
    let mut _result = format!("Result: ");

    // Start
    println!("Rust Main Starting...\n");
    println!("Running through Rust the Programming Language concepts.\n");

    println!("1. Updating a variable");
    let mut input = 1;
    mutate_variable(&mut input); 
    println!("let x = {};", input);
    println!("We can update x by shadowing with: let x = 2;");
    println!("But we cannot change x otherwise since it is immutable with: let x =");
    println!("We could rewrite x however, by letting it be mutable: let mut x = 0;");
    println!("Now we can update x: x = 1;");
    println!("\n");

    println!("2. Setting a constant");
    give_a_constant();

    println!("3. Shadowing variables");
    give_shadowing_update();

    println!("4. Writing to a buffer to format a string");
    let mut buf = String::new(); 
    let _ = mutate_buf_with_fmt(1, &mut buf);
    println!("Result: {:?}\n\n", buf);

    println!("5. Using expressions");
    calc_add(2, 2);

    println!("6. Using conditional expressions");
    calc_conditional_expression(3, 4);
    println!("\n");

    println!("6. Using statements");
    println!("\n");

    println!("7. Using multiple conditionals");
    calc_wrap_around_conditional(10);
    println!("\n");

    println!("8. Using if let");
    println!("Result: {:?}\n", calc_if_let(1));

    println!("9. Using conditional loop");
    println!("Result: {:?}\n", calc_conditional_loop_count(5));

    println!("10. Where do literal strings go (TBD)?\n");

    println!("11. Examples of ownership in functions");
    let mut s = String::from("this string");
    println!("We create a String: {:?}", s);
    let moved_s = std::mem::take(&mut s);
    println!("Now we pass the string to function not as a reference or borrow.");
    take_string_ownership(moved_s);
    println!("Result: {:?}\n", s);

    let mut x = 5;
    println!("x is: {:?}", x);
    println!("Now we copy x to y.");
    let mut y = take_copy(x);
    println!("y is: {:?}", y);
    x = 10; 
    println!("We modify x: {:?}", x); 
    println!("y is still: {:?}", y);
    y = 20;
    println!("We modify y: {:?}", y);
    println!("Now we have x:{:?}, y:{:?}\n", x,y);

    println!("12. Giving ownership from a function");
    let give_me = give_ownership();
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
    let slice = give_first_word_slice(&my_words);
    println!("Slice: {:?}\n", slice);
    let another_slice = &my_words[0..7];
    println!("Another slice: {:?}\n", another_slice);

    println!("16. Better control flow with \"it let\"");
    let mut data = Data::new();
    let input = (DataAction::Write, Some("Adding this block."));
    println!("Writing data with exhaustive match pattern.");
    let result = take_verbose_control_flow_string(&mut data, input);
    println!("Result: {:#?}", result);
    println!("Now writing data with concise \"if let\".");
    let input = (DataAction::Write, String::from("Adding this block."));
    let result = mutate_only_control_flow(&mut data, input);
    println!("Result: {:#?}\"", result);
    println!("Reading data.");
    let input = (DataAction::Read, None);
    let result = take_verbose_control_flow_string(&mut data, input);
    println!("Result: {:#?}\n", result);

    println!("17. Handling collections\n");

    println!("Creating a vector.");
    let mut vector = give_vector_with_capacity(Some(1));
    println!("Now modifying.");
    let result = mutate_vector(&mut vector, 1);
    // result = vector_modify(&mut vector, 1);
    println!("Result: {:#?}\n", result);
    println!("Trying to read something with no value.");
    let result = vector.get(3); 
    println!("Result: {:#?}", result);

    let mut string = String::from("cat");
    println!("Transforming a String (char array): {}", string);
    let result = mutate_string_collection_to_vertical(&mut string);
    println!("Result: \n{}\n", result);

    println!("Creating and handling hash maps.\n");
    println!("Creating a new hash map.");
    let mut hashmap = give_hashmap();
    println!("Result: {:#?}\n", hashmap);
    println!("Mutating hashmap.");
    let entry = (String::from("Key1"), 42);
    mutate_hashmap(&mut hashmap, entry);
    println!("Result: {:#?}\n", hashmap);
    println!("Overwriting existing hash map entry.");
    let entry = (String::from("Key1"), 99);
    mutate_hashmap(&mut hashmap, entry);
    println!("Result: {:#?}\n", hashmap);
    println!("Adding new hash map pair (grows like a stack).");
    let entry = (String::from("Key2"), 42);
    mutate_hashmap(&mut hashmap, entry);
    println!("{} {:#?}\n", _result, hashmap);
    println!("Trying to update an existing key-value.");
    let entry = (String::from("Key2"), 99);
    try_mutate_hashmap(&mut hashmap, entry);
    println!("{} {:#?}\n", _result, hashmap);


    println!("18. Error handling\n");
    println!("Trying panic.");
    let real_panic = false;
    let did_real_panic: Result<(), Error> = match real_panic { 
        true => {
            println!("Executing real panic.\n"); 
            let _ = try_take_panic(real_panic);
            Err(Error::last_os_error())
        }
        false => try_take_panic(real_panic) 
    };
    println!("Result: mock_program_panic({})\n", did_real_panic.is_ok());

    Ok(())
}


pub type EmptyString<'a> = &'a str;
#[allow(unused)]
const EMPTY_STR: EmptyString = "";

pub type PrettyFormat<'a> = &'a str;
#[allow(unused)]
const FMT: PrettyFormat = "{:#?}"; 