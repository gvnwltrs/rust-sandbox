use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

#[allow(unused)]
use rust_main::*;


fn main() -> Result<(), Error> {
    use PType::*;

    // Start
    msg(Desc, "Rust Main Starting...\n");
    msg(Desc, "Running through Rust the Programming Language concepts.\n");

    msg(Desc, "1. Updating a variable");
    let mut input = 1;
    mutate_variable(&mut input); 
    msg(Desc, "let x = <input>;");
    msg(Data, &input);
    msg(Desc, "We can update x by shadowing with: let x = 2;");
    msg(Desc, "But we cannot change x otherwise since it is immutable with: let x =");
    msg(Desc, "We could rewrite x however, by letting it be mutable: let mut x = 0;");
    msg(Desc, "Now we can update x: x = 1;\n");

    msg(Desc, "2. Setting a constant");
    give_a_constant();
    msg(Desc, "We can set a constant by: const X: u32 = 0;");
    msg(Desc, 
        "
        But this is different than variables because we 
        MUST use a type annotation such as: const X: u32
        \n"
    );

    msg(Desc, "3. Shadowing variables");
    give_shadowing_update();
    msg(Desc, "We can perform shadowing by simply re-assigning a previously used variable alias:
        let x = 0;
        let x = 1;\n"
    );
    msg(Desc, "In the previous case, it was a little pointless since we are using the same type.");
    msg(Desc, "To gain the real value from shadowing, we can use it as an alternative to typecasting:
        let x = \"change to a string\";\n"
    );
    msg(Desc, "Now x has gone from an integer to a string type (&str more precisely)\n");
    msg(Desc, "\n");

    msg(Desc, "4. Writing to a buffer to format a string");
    let mut buf = String::new(); 
    let _ = mutate_buf_with_fmt(1, &mut buf);
    msg(Res, &buf);

    msg(Desc, "5. Using expressions");
    derive_add(2, 2);
    msg(Desc, "We can do an expression by: let out = {{ a + b }};");
    msg(Desc, "Or we can do it shorthand by: let out = a + b;");
    msg(Desc,
        "
        In most cases, whatever you decide to use is a matter of preferrence
        But it seems like the braces way indicates an expression more explicitly...
        \n"
    );

    msg(Desc, "6. Using conditional expressions");
    derive_conditional_expression(3, 4);
    msg(Desc,
        "
        We can do common conditional expressions using if, else if, and else:
            if a == b {{
                msg(\"a is equal to b\");
            }} else if a < b {{
                    msg(\"a is less than b\");
            }} else {{
                msg(\"a is not greater than b\");
            }}
        "
    );
    msg(Desc, "a is equal to b");
    msg(Desc, "a is less than b");
    msg(Desc, "a is not greater than b\n");

    msg(Desc, "6. Using statements\n");

    msg(Desc, "7. Using multiple conditionals");
    derive_wrap_around_conditional(10);
    msg(Desc, "Handling multiple conditionals with if-else.\n");

    msg(Desc, "8. Using if let");
    msg(Res, &derive_if_let(1));

    msg(Desc, "9. Using conditional loop");
    msg(Res, &derive_conditional_loop_count(5));
    msg(Desc, "REMEMBER: While loops are great counters or countdowns. Not for collections though...");
    msg(Desc, "Also, even for countdowns or counters, a for loop might be a better option since it sets clear boundaries...");

    msg(Desc, "10. Where do literal strings go (TBD)?\n");

    msg(Desc, "11. Examples of ownership in functions");
    let mut s = String::from("this string");
    msg(Desc, "We create a String:");
    msg(Data, &s);
    let moved_s = std::mem::take(&mut s);
    msg(Desc, "Now we pass the string to function not as a reference or borrow.");
    take_string_ownership(moved_s);
    msg(Res, &s);

    let mut x = 5;
    msg(Res, &x);
    msg(Desc, "Now we copy x to y.");
    let mut y = take_copy(x);
    msg(Res, &y);
    x = 10; 
    msg(Desc, "We modify x"); 
    msg(Res, &x);
    msg(Res, &y);
    y = 20;
    msg(Desc, "We modify y");
    msg(Desc, "Now we have this for x and y:\n");
    msg(Res, &x);
    msg(Res, &y);

    msg(Desc, "12. Giving ownership from a function");
    let give_me = give_ownership();

    msg(Desc, "We call 'gives_ownership' and it gives us a string:\n");
    msg(Data, &give_me);
    msg(Res, &give_me);

    msg(Desc, "13. Mutable references"); 
    let mut my_string = String::from("Before we had this...");
    msg(Data, &my_string);
    mutate_reference(&mut my_string);
    msg(Desc,
        "\
        We can modify a reference so long as we claim that's \
        what we are doing with the thing we borrow: \
        \n"
    );
    msg(Res, &my_string);

    msg(Desc, "14. Dangling references (TBD)...\n");

    msg(Desc, "15. String slices");
    let my_words = String::from("This is it");
    msg(Data, &my_words);
    let slice = access_first_word_slice(&my_words);
    msg(Res, &slice);
    let another_slice = &my_words[0..7];
    msg(Res, &another_slice);

    msg(Desc, "16. Better control flow with \"it let\"");
    let mut data = TData::new();
    let input = (DataAction::Write, Some("Adding this block."));
    msg(Desc, "Writing data with exhaustive match pattern.");
    let result = take_verbose_control_flow_string(&mut data, input);
    msg(Res, &result);
    msg(Desc, "Now writing data with concise \"if let\".");
    let input = (DataAction::Write, String::from("Adding this block."));
    let result = mutate_only_control_flow(&mut data, input);
    msg(Res, &result);
    msg(Desc, "Reading data.");
    let input = (DataAction::Read, None);
    let result = take_verbose_control_flow_string(&mut data, input);
    msg(Res, &result);

    msg(Desc, "17. Handling collections\n");

    msg(Desc, "Creating a vector.");
    let mut vector = give_vector_with_capacity(Some(1));
    msg(Desc, "Now modifying.");
    let result = mutate_vector(&mut vector, 1);
    // result = vector_modify(&mut vector, 1);
    msg(Res, &result);
    msg(Desc, "Trying to read something with no value.");
    let result = vector.get(3); 
    msg(Res, &result);

    let mut string = String::from("cat");
    msg(Desc, "Transforming a String (char array):");
    msg(Data, &string);
    let result = mutate_string_collection_to_vertical(&mut string);
    msg(Res, &result);

    msg(Desc, "Creating and handling hash maps.\n");
    msg(Desc, "Creating a new hash map.");
    let mut hashmap = give_hashmap();
    msg(Res, &hashmap);
    msg(Desc, "Mutating hashmap.");
    let entry = (String::from("Key1"), 42);
    mutate_hashmap(&mut hashmap, entry);
    msg(Res, &hashmap);
    msg(Desc, "Overwriting existing hash map entry.");
    let entry = (String::from("Key1"), 99);
    mutate_hashmap(&mut hashmap, entry);
    msg(Res, &hashmap);
    msg(Desc, "Adding new hash map pair (grows like a stack).");
    let entry = (String::from("Key2"), 42);
    mutate_hashmap(&mut hashmap, entry);
    msg(Res, &hashmap);
    msg(Desc, "Trying to update an existing key-value.");
    let entry = (String::from("Key2"), 99);
    try_mutate_hashmap(&mut hashmap, entry);
    msg(Res, &hashmap);
    msg(Desc, "Does not mutate because key and value already populated.\n");

    msg(Desc, "18. Error handling\n");
    msg(Desc, "Trying panic.");
    let real_panic = false;
    let did_real_panic: Result<(), Error> = match real_panic { 
        true => {
            println!("Executing real panic.\n"); 
            let _ = try_take_panic(real_panic);
            Err(Error::last_os_error())
        }
        false => try_take_panic(real_panic) 
    };
    msg(Impl, "mock_program_panic()\n");
    msg(Res, did_real_panic.is_ok());

    msg(Desc, "19. Generics\n");
    msg(Desc, "Finding largest value in collection with generic function.");
    msg(Impl, "access_largest(&[T])");
    let data = vec![0, 1, 2, 3];
    msg(Data, &data);
    let largest = access_largest(&data);
    msg(Res, &largest);

    msg(Desc, "Using a generic multi-type struct.");
    let _input = (42, 42.0, '*');
    msg(Data, &_input);
    msg(Impl, "give_typed_struct((T, U, V))");
    let result = give_typed_struct(_input);
    msg(Res, &result);

    Ok(())
}

#[derive(Debug)]
enum PType {
    Desc,
    Impl,
    Data,
    Res,
}

fn msg<T: std::fmt::Debug>(t: PType, msg: T) {
    // Message helpers
    match t {
        PType::Desc => println!("Description: {:#?}", msg),
        PType::Impl => println!(" | function: {:#?}", msg),
        PType::Data => println!(" | data: {:#?}", msg),
        PType::Res => println!(" | result: {:#?}", msg),
    }
}

pub type EmptyString<'a> = &'a str;
#[allow(unused)]
const EMPTY_STR: EmptyString = "";

pub type PrettyFormat<'a> = &'a str;
#[allow(unused)]
const FMT: PrettyFormat = "{:#?}"; 