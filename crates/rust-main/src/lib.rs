use rand::prelude::*;
use core::fmt::Write;

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

/* Variables & Mutability */
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
    println!("END\n");
}

pub fn set_a_constant() {
    println!("We can set a constant by: const X: u32 = 0;");
    const _X: u32 = 0;
    println!("
        But this is different than variables because we 
        MUST use a type annotation such as: const X: u32"
    );

    println!("END\n");
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

    println!("END\n");
}

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
    println!("END\n");
    out
}

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