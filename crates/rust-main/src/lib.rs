/* rust-main::lib.rs */

use rand::prelude::*;
use core::fmt::Write;

#[allow(unused)]
use std::io::{Error, ErrorKind};
//use std::fmt::Error;

#[allow(unused)]
use std::result::Result;
//use core::fmt::Result;

#[allow(unused)]
use chrono::{Local, Utc};

#[allow(unused)]
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct Assignments {
    x: Option<char>,
    y: Option<char>,
    value: Option<i32>,
}

impl Assignments {
    pub fn new() -> Self {
        Self { 
            x: None,
            y: None,
            value: None,
        }
    }

    pub fn one(&mut self, a: char, val: i32) {
        self.x = Some(a);
        self.value = Some(val);
    }
}

/* 0) Three core discplines of Rust and in programming generally */

// 1. Ownership

// 2. Borrowing

// 3. Lifetimes

/* 1) Variables and mutability */

// Why? Shadowing allows you to change the type that a var name 
// holds, but still allowing for a reuse of the name without having
// to do somethign like a typecast. Think healthier alternative to typecasting. 
pub fn give_shadowed_assignments() -> Assignments {
    let _val = "?";
    let _val = 2.0;
    let _val = 3;
    let mut a = Assignments::new(); 
    a.one('a', _val);
    a
} 

pub fn mutate_borrow_checker() {
    let mut x = Box::new(42); // heap allocated; think of Box == smart_pointer
    let r = &x; // 'a -- we know that x lives outside of this slot

    if rand::rng().random::<f64>() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r); // 'a
    }
}

// Trying write formats
pub fn mutate_buf_with_fmt<W: Write>(_in: i32, out: &mut W) -> core::fmt::Result {
    // unimplemented!();
    write!(out, "value={}...", _in)
}

pub fn mutate_variable(input: &mut i32) -> &mut i32 {
    *input = 1;
    *input = 2;
    *input = 0;
    *input = 1;
    input 
}

pub fn give_a_constant() {
    println!("We can set a constant by: const X: u32 = 0;");
    const _X: u32 = 0;
    println!("
        But this is different than variables because we 
        MUST use a type annotation such as: const X: u32"
    );

    println!("\n");
}

pub fn give_shadowing_update() {
    println!("We can perform shadowing by simply re-assigning a previously used variable alias:
        let x = 0;
        let x = 1;\n"
    );
    let _x = 0;
    let _x = 1;
    println!("In the previous case, it was a little pointless since we are using the same type.");
    println!("To gain the real value from shadowing, we can use it as an alternative to typecasting:
        let x = \"change to a string\";\n"
    );
    println!("Now x has gone from an integer to a string type (&str more precisely)\n");

    println!("\n");
}

/* 2) Data types */


/* 3) Functions */

// Math expressions
pub fn calc_add(a: i32, b: i32) -> i32 {
    println!("We can do an expression by: let out = {{ a + b }};");
    let _out = { a + b }; // is it preferrable to use braces to signal what is an expression?
    println!("Or we can do it shorthand by: let out = a + b;");
    let out = a + b;
    println!("
        In most cases, whatever you decide to use is a matter of preferrence
        But it seems like the braces way indicates an expression more explicitly...
        "
    );
    println!("\n");
    out
}

/* 4) Comments */

/* 5) Control flow */

// Conditional expressions
pub fn calc_conditional_expression(a: i32, b: i32) -> bool {
    println!("
        We can do common conditional expressions using if, else if, and else:
            if a == b {{
                println!(\"a is equal to b\");
            }} else if a < b {{
                    println!(\"a is less than b\");
            }} else {{
                println!(\"a is not greater than b\");
            }}"
    );
    if a == b {
        println!("a is equal to b");
        true
    } else if a < b {
        println!("a is less than b");
        true
    } else {
        println!("a is not greater than b");
        false
    }
}

/* 6) Loops */

pub fn calc_wrap_around_conditional(start: i32) -> bool {
    println!("Handling multiple conditionals with if-else.");
    if start < 3 { println!("Come on...{:#?}", start); return false};
    let mut count = 1;
    let begin = (start+1) % start ;
    let mid = (start/2) % start;
    let end = (start-1) % start;
    while count > 0 {
        if count % start == begin {
            println!("Currently at: {:#?}", count);
        } else if count % start == mid {
            println!("Getting closer: {:#?}", count);
        } else if count % start == end {
            println!("We made it! {:#?}", count+1);
            count = 0;
            continue;
        } else {
            println!("We're somewhere: {:#?}", count);
        }  
        count += 1;
    }

    true
}

pub fn calc_if_let(num: i32) -> bool {
    let _case = if num == 1 {true} else{false};
    _case
}

pub fn calc_conditional_loop_count(num: i32) -> i32 {
    println!("REMEMBER: While loops are great counters or countdowns. Not for collections though...");
    println!("Also, even for countdowns or counters, a for loop might be a better option since it sets clear boundaries...");
    let mut counter = num;
    while counter != 0 {
        println!("{:#?}", counter);
        counter -= 1;
    }
    counter
}

/* 7) Ownership */ 

// FIXME: Broken...
// pub fn give_string_mem_location(string: &str) -> &'static str {
//     let mut _string = string;
//     let mut buffer = [0u8; 32];
//     if _string == "" {
//         let text = "stack-text";
//         let len = text.len();
//         buffer[..len].copy_from_slice(text.as_bytes());
//         _string = std::str::from_utf8(&buffer[..len]).unwrap();
//     };

//     let stack_val = 0;
//     let stack_addr = &stack_val as *const _ as usize;
//     let heap_addr = Box::into_raw(Box::new(0)) as usize;
//     let literal_addr = _string.as_ptr() as usize;
//     let f_addr = where_does_this_string_live as *const () as usize;

//     let dist_to_f = (literal_addr as isize - f_addr as isize).abs();
//     let dist_to_stack = (literal_addr as isize - stack_addr as isize).abs();
//     let dist_to_heap = (literal_addr as  isize - heap_addr as isize).abs();

//     if  dist_to_stack < dist_to_f && dist_to_stack < dist_to_heap {
//        "stack"
//     } else if dist_to_f < dist_to_heap {
//         ".rodata (near fn)"
//     } else {
//         "heap"
//     }
// }

/*  8) Ownership & functions  */

pub fn take_string_ownership(s: String) {
    let _s = s;
}

pub fn take_copy(val: i32) -> i32 {
    let _val = val;
    _val
}

/* 9) Borrowing & references */

pub fn give_ownership() -> String {
    String::from("Here, have a string. You own it!")
}

/* 10) Mutable references */

pub fn mutate_reference(s: &mut String) {
    s.push_str("and now we have this...");
}

/* 11) Dangling references */

// Slice types 
pub fn give_first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/* 12) Better control flow */

#[allow(unused)]
#[derive(Debug)]
pub struct Data {
    text_blocks: Vec<String>,
}

impl Data {

    pub fn new() -> Self { Self { text_blocks: vec![], }}

}

#[allow(unused)]
#[derive(Debug)]
pub enum DataAction {
    Read,
    Write,
    None,
}

pub fn take_verbose_control_flow_string(
    data: & mut Data, 
    input: (DataAction, Option<&str>)) -> Option<String> {
    match input {
        (DataAction::Read, None)  => { 
            Some(data.text_blocks
                .iter()
                .map(|block| block.as_str())
                .collect()
            )
        },
        (DataAction::Write, Some(text)) => {
            data.text_blocks.push(text.to_string());
            Some(String::from("Wrote text block."))
        },
        _ => None
    }
}

// Only write
pub fn mutate_only_control_flow(data: &mut Data, input: (DataAction, String)) -> Option<String> {
    if let DataAction::Write = input.0 {
        data.text_blocks.push(input.1);
        Some(String::from("Wrote text block."))
    } else {
        None
    }  
}

/* 13) Collections from standard library */

#[derive(Debug)]
pub enum Command {
    None,
    Alter,
}

pub fn take_tuple(num: u32, msg: String, _cmd: Option<Command>) -> (u32, String) {
    (num, msg)
}

pub fn take_tuple_val(item: &(u32, String)) -> String {
    format!("val: {}, msg: {}", item.0, item.1)
}

pub fn give_vector_with_capacity(cap: Option<usize>) -> Vec<u32> {
   match cap {
    Some(x) => Vec::with_capacity(x),
    None => Vec::new()
   }
}

pub fn mutate_vector(v:&mut Vec<u32>, val: u32) -> Result<u32, Error> {
    if v.capacity() > 0 && v.len() == v.capacity() {
        return Err(ErrorKind::OutOfMemory.into()); 
    }

    v.push(val); 
    Ok(val)
}

pub fn mutate_string_collection_to_vertical(string: &mut String) -> &str {
    let mut out = String::new();
    for c in string.chars() {
        out.push(c);
        out.push('\n');
    }
    *string = out;
    string
} 

pub fn give_hashmap() -> HashMap<String, i32> {
    let mut _hashmap = HashMap::new();
    _hashmap
}

pub fn mutate_hashmap(map: &mut HashMap<String, i32>, entry: (String, i32)) {
    map.insert(entry.0, entry.1);
}

pub fn try_mutate_hashmap(map: &mut HashMap<String, i32>, entry: (String, i32)) -> Option<&mut i32> {
    match map.entry(entry.0) {
        Entry::Occupied(occupied) => { Some(occupied.into_mut()) }
        Entry::Vacant(vacant) => { vacant.insert(entry.1); None } 
    }
    // Some(map.entry(entry.0).or_insert(entry.1))
}

/* 14) Error Handling */

pub fn try_take_panic(enable: bool) -> Result<(), Error> {
    match enable {
        true => panic!("Crash test."),
        false => Ok(())
    }
}

#[cfg(test)]
mod rust_main_tests {
    #[allow(unused)]
    use super::*;

    // Basics 

   #[test]
   fn test_give_shadowed_assignments() {
    let s_struct = give_shadowed_assignments();
    assert!(s_struct == Assignments { x:Some('a') , y: None, value: Some(3) });
   } 

    // Collections

    #[test]
    fn test_take_tuple() {
        let tuple = take_tuple(42, String::from("Tuple is on stack."), None); 
        assert!(tuple == (42, String::from("Tuple is on stack.")));
    }

    #[test]
    fn test_stack_take_tuple_val() {
        let tuple = take_tuple(42, String::from("Tuple is on stack."), None); 
        let result = take_tuple_val(&tuple);
        assert!(result == String::from("val: 42, msg: Tuple is on stack."));
    }

    #[test]
    fn test_give_vector() {
        let vector = give_vector_with_capacity(None); 
        assert!(vector == Vec::new());
    }

    #[test]
    fn test_modifying_a_vector() {
        let mut vector = give_vector_with_capacity(None);
        let _ = mutate_vector(&mut vector, 1);
        let _ = mutate_vector(&mut vector, 2);
        let _ = mutate_vector(&mut vector, 3);
        assert!(vector == vec![1, 2, 3]);
    }

    #[test]
    fn test_reading_a_vector_value() {
        let mut vector = give_vector_with_capacity(None);
        let _ = mutate_vector(&mut vector, 1);
        let _ = mutate_vector(&mut vector, 2);
        let _ = mutate_vector(&mut vector, 3);
        let read = &vector[2];  // reference is optional, but no need to copy
        assert!(*read == 3);
    }

    #[test]
    fn test_reading_a_vector_with_no_value() {
       let mut vector = give_vector_with_capacity(Some(10)); 
       let _ = mutate_vector(&mut vector, 1);
       let read = vector.get(2);
       assert!(read.is_none());
    }

    #[test]
    fn test_string_collection_iteration() {
        let mut string = String::from("cat");
        let result = mutate_string_collection_to_vertical(&mut string);
        assert!(result == String::from("c\na\nt\n"));
    }

    #[test]
    fn test_give_hashmap() {
        let hashmap = give_hashmap();
        assert!(hashmap == HashMap::new());
    }

    #[test]
    fn test_mutate_hashmap() {
        let mut hashmap = give_hashmap();
        let entry = (String::from("new entry"), 42);
        let _ = mutate_hashmap(&mut hashmap, entry);
        let found = hashmap.get("new entry"); 
        assert!(found == Some(&42));
    }

    #[test]
    fn test_try_mutate_hashmap() {
        let mut hashmap = give_hashmap();
        let entry = (String::from("new entry"), 42);
        let result = try_mutate_hashmap(&mut hashmap, entry); // "takes" entry (goes out of existence)
        assert!(result.is_none());

        let entry = (String::from("new entry"), 99); // alread has a value so does nothing
        let result = try_mutate_hashmap(&mut hashmap, entry);
        assert!(result.is_some());
    }

    // Error handling

    #[test]
    fn test_try_panic() {
        let did_panic = try_take_panic(false);
        assert!(did_panic.is_ok());
    }

}