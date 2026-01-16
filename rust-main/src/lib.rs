
use rand::prelude::*;

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