
/* 
 * Object-Oriented Programming in Rust 
 **/

// Object-Oriented approach
pub struct MyObject {
    a: u32,
    b: f32,
    c: String,
}

// Sets policy for how MyObject gets implemented
pub trait MyObjectTrait {
    fn new(a: u32, b: f32, c: String) -> Self;
    fn get_a(&self) -> u32;
    fn get_b(&self) -> f32;
    fn get_c(&self) -> String;
}

// Implentation given struct and trait policy/contract
impl MyObjectTrait for MyObject {
   fn new(a: u32, b: f32, c: String) -> Self {
        MyObject {
            a: a,
            b: b,
            c: c,
        }
    }

    fn get_a(&self) -> u32 {
        self.a
    }

    fn get_b(&self) -> f32 {
        self.b
    }

    fn get_c(&self) -> String {
        self.c.clone()
    }
}

/* 
 * Mixing OOP with FP in Rust 
 **/

// Adding functional programming to object
pub trait MyObjectApply {
    fn apply<F, R>(&self, f: F) -> R 
    where F: Fn(u32, f32, String) -> R;

    fn execute_one<F, R, T>(&self, f: F, info: T) -> R 
    where F: Fn(T) -> R;
}

// Once constructor has been called, this can operate on what's initialized by it.
// This is a "higher-order function" in that it turns a function into a variable
// so that it can be used and reused as a variable. The function that is doing 
// the calling on the input function is the higher-order function. Functions in 
// Rust are first-class functions by default since they can be stored or treated
// as a variable, such as in the case of passing one into the higher-order function.
impl MyObjectApply for MyObject {
    fn apply<F, R>(&self, f: F) -> R 
    where F: Fn(u32, f32, String) -> R {
        f(self.a, self.b, self.c.clone()) // 1st class function
    }

    fn execute_one<F, R, T>(&self, f: F, info: T) -> R 
    where F: Fn(T) -> R {
        f(info) // 1st class function
    }

}
