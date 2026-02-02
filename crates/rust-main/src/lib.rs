use rand::prelude::*;
use core::fmt::Write;
#[allow(unused)]
use std::io::Error;

#[allow(unused)]
use core::fmt::Result;

#[derive(Debug)]
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

// 0) Three core discplines of Rust and in programming generally

// 1) Variables and mutability

// Why? Shadowing allows you to change the type that a var name 
// holds, but still allowing for a reuse of the name without having
// to do somethign like a typecast. Think healthier alternative to typecasting. 
pub fn shadowing() -> Assignments {
    let _val = "?";
    let _val = 2.0;
    let _val = 3;
    let mut a = Assignments::new(); 
    a.one('a', _val);
    a
} 

pub fn borrow_checker() {
    let mut x = Box::new(42); // heap allocated; think of Box == smart_pointer
    let r = &x; // 'a -- we know that x lives outside of this slot

    if rand::rng().random::<f64>() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r); // 'a
    }
}

// Trying write formats
pub fn write_fmt_to_buf<W: Write>(_in: i32, out: &mut W) -> core::fmt::Result {
    // unimplemented!();
    write!(out, "value={}...", _in)
}

pub fn updating_a_variable() {
    println!("let x = 1;");
    let _x = 1;
    println!("We can update x by shadowing with: let x = 2;");
    let _x = 2;
    println!("But we cannot change x otherwise since it is immutable with: let x =");
    println!("We could rewrite x however, by letting it be mutable: let mut x = 0;");
    let mut _x = 0;
    println!("Now we can update x: x = 1;");
    _x = 1;
    let _ = _x;
    println!("\n");
}

pub fn set_a_constant() {
    println!("We can set a constant by: const X: u32 = 0;");
    const _X: u32 = 0;
    println!("
        But this is different than variables because we 
        MUST use a type annotation such as: const X: u32"
    );

    println!("\n");
}

pub fn performing_shadowing() {
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

// 2) Data types


// 3) Functions

// Math expressions
pub fn add_expressions(a: i32, b: i32) -> i32 {
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

// 4) Comments

// 5) Control flow

// Conditional expressions
pub fn conditional_expression(a: i32, b: i32) -> bool {
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

// 6) Loops

pub fn wrap_around_conditional(start: i32) -> bool {
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

pub fn if_let(num: i32) -> bool {
    let _case = if num == 1 {true} else{false};
    _case
}

pub fn conditional_loop(num: i32) -> i32 {
    println!("REMEMBER: While loops are great counters or countdowns. Not for collections though...");
    println!("Also, even for countdowns or counters, a for loop might be a better option since it sets clear boundaries...");
    let mut counter = num;
    while counter != 0 {
        println!("{:#?}", counter);
        counter -= 1;
    }
    counter
}

// 7) Ownership 

pub fn where_does_this_string_live(string: &str) -> &'static str {
    let mut _string = string;
    let mut buffer = [0u8; 32];
    if _string == "" {
        let text = "stack-text";
        let len = text.len();
        buffer[..len].copy_from_slice(text.as_bytes());
        _string = std::str::from_utf8(&buffer[..len]).unwrap();
    };

    let stack_val = 0;
    let stack_addr = &stack_val as *const _ as usize;
    let heap_addr = Box::into_raw(Box::new(0)) as usize;
    let literal_addr = _string.as_ptr() as usize;
    let f_addr = where_does_this_string_live as *const () as usize;

    let dist_to_f = (literal_addr as isize - f_addr as isize).abs();
    let dist_to_stack = (literal_addr as isize - stack_addr as isize).abs();
    let dist_to_heap = (literal_addr as  isize - heap_addr as isize).abs();

    if  dist_to_stack < dist_to_f && dist_to_stack < dist_to_heap {
       "stack"
    } else if dist_to_f < dist_to_heap {
        ".rodata (near fn)"
    } else {
        "heap"
    }
}

// 8) Ownership & functions 

pub fn takes_ownership(s: String) {
    let _s = s;
}

pub fn makes_copy(val: i32) -> i32 {
    let _val = val;
    _val
}

// 9) Borrowing & references

pub fn gives_ownership() -> String {
    String::from("Here, have a string. You own it!")
}

// 10) Mutable references

pub fn mutate_reference(s: &mut String) {
    s.push_str("and now we have this...");
}

// 11) Dangling references

// Slice types 
pub fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[derive(Debug)]
#[allow(unused)]
pub struct MyStruct {
    x: i32,
    y: i32,
}

#[derive(Debug)]
#[allow(unused)]
pub struct TupleStruct(i32, i32);

pub fn init_struct() -> MyStruct {
    MyStruct { x: 42, y: 43 }
}

pub fn init_tup_struct(x: i32, y: i32) -> TupleStruct {
    TupleStruct(x, y)
}

#[derive(Debug)]
#[allow(unused)]
pub struct MyData{
    values: i32,
}

impl MyData {
    pub fn new() -> Self {
        Self {
            values: 42,
        }
    }

    pub fn reset(&mut self) {
        self.values = 42;
    }
}

pub fn using_data_method(data: &mut MyData) {
    data.values = 100;
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Vault {
    Name(String),
    ID(i32),
}

pub fn update_vault(vault: &mut Vault, val: &Vault) {
    *vault = val.clone();
} 

// Returns by giving ownership to a string (move)
pub fn read_vault(vault: &Vault) -> String {
    let mut msg = String::from("Type:"); 

    match vault {
        Vault::Name(name) => { 
            let variant = String::from(" String, ");
            msg.push_str(variant.as_str());
            msg.push_str(name)
        },
        Vault::ID(id) => {
            let variant = String::from(" i32, ");
            msg.push_str(variant.as_str());
            msg.push_str(id.to_string().as_str());
        }
    }
    msg
}

// Exploring "kinds" with enums 
#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Operation {
    Quit(String), // Contains a "quit" message
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn execute_op(op: &Operation) -> String {
    let mut msg = String::default();
    match op {
        Operation::Quit(_) => {println!("Quit operation executing.");}
        Operation::Move { .. } => { println!("Move operation executing."); }
        Operation::Write(_msg) => { 
            println!("Write operation executing."); 
            msg.push_str(_msg); 
        }
        Operation::ChangeColor(_, _, _) => { println!("Change color operation executing."); }
    }
    msg
}
// Using braces {} for match arms—even when they only contain a single expression—is a
// common practice: it makes the code consistent and much easier to extend later. Since
// the match is already exhaustive, it just needs to tidy up the unreachable code to make 
// the compiler happy. 

#[cfg(test)]
mod rust_main_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_sanity() {
        assert!(true);
    }

    #[test]
    fn test_execute_operation() {
        let write = Operation::Write(String::from("Append"));
        let result = execute_op(&write);
        let expected = String::from("Append");

        assert_eq!(result, expected);
    }
}