
/* 
* Intro to Functional Programming in Rust 
**/

// This is a "pure function" because it does not modify the original, or
// the input "x: T". It only use a read-only of "x" to perform a calculation
// and returns the result of that calculation which is then owned by the caller.
pub fn square<T>(x: T) -> T 
where T: std::ops::Mul<Output=T> + Copy
{  
    x*x
}

// This is NOT a "pure function". This is "impure" or a "side-effect"
// because it uses println! which is an IO call to the external system
// or OS.
pub fn tupled(x: i32, y: f64, z: String) -> (i32, f64, String) {
    type Tuple1 = (i32, f64, String);

    let this: Tuple1 = (x, y, z); 
    println!("output: {:#?}", this);    
    this
}

// This is still a "pure function" because we only return a ref
// that points backto the source of the ref. Nothing is modified 
// in the original.
pub fn get_one(collection: &Vec<String>) -> Option<&String> {
    match collection.get(0) {
        Some(x) => Some(x),
        _ => None
    } 
}

// This is a "pure function" because it only reads then points to a specific segment 
// of the original. In this case we only look-at/read the ref to select
// a section to return as a ref which points to a slice of the original. 
pub fn get_one_char<'a>(collection: &'a Vec<String>, c: usize) -> &'a str {
    let s = &collection[c];
    &s[0..1]
}

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

/* 
 * Functional Programing patterns in Rust 
 **/

// Explicit pure functions in FP

// Pure because it is completely isolated and within the safe zone or boundary
// of the application.
pub fn pure_function1(x: u32) -> u32 {
    x * x
}

// Explicit non-pure or "side-effect" functions

// Impure because it uses something outside the scope of the function 
// implicity, or outside the boundary of what would be considered a 
// "safe zone", meaning an external dependencies or operation that could
// fail outside of the control or boundary of the safe zone. A print statement
// depends on the native OS to perform the output.  
pub fn impure_function(x: u32) -> u32 {
    println!("x = {}", x);
    x * x
}

// Immutability patterns

// Owner of the tuple will be the caller
pub fn immutable_operation1() -> (i32, i32) {
    let immutable_v1 = 1;
    // immutable_v1 = 2; //invalid

    let mut mutable_v2 = 1;
    mutable_v2 = 2;

    (immutable_v1, mutable_v2)
}

// Functional composition:
// Uses output of one function as input to another function
// by being connected. 
pub fn functional_composition(x: f64) -> f64{
    let fsin = |X: f64| x.sin();
    let fabs = |x: f64| x.abs();
    // Feed ouput of one into the other
    let transform = |x: f64| fabs(fsin(x));

    transform(x)
}

// Higher-order function again...
// A predicate means that we have a function
// returning either "true" or "false".
pub fn is<F>(predicate: F, x: u32) -> bool 
where F: Fn(u32) -> bool {
    predicate(x)
}

// Functor
// In layman's terms, a functor is anything you can map over. 
// So you use them all the time without realizing it. 
// It does not mutate data, it only transforms. So this example
// shows how you do that by adding a number to a letter without 
// changing the letter. The number being the index for the original
// value now. 
pub fn functor() -> Vec<(char, u32)> {
    let c = 0;
    vec!['a', 'b', 'c']
        .into_iter()
        .scan(c, |c0, letter| {
            *c0 += 1; 
            Some((letter, *c0))
        }) 
        .collect()
}

// Monads
// For simplicity, a monad is just a trait with two methods.
//
// A monad is almost like a self-contained thing that returns 
// a binding to something. 
//
// A monad doesn’t return a binding 
// — it returns another value in the same container/context.
//
// A monad is a self-contained context that defines how computations 
// are chained while carrying an effect.
//
// A monad is a container that carries a value and defines how to safely 
// chain computations that produce values in the same container.
pub trait Monad: Sized {
    type Item;
    type Wrapped<B>; // "same container, different inner type"

    fn pure(x: Self::Item) -> Self;
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Item) -> Self::Wrapped<B>;
}

impl<T> Monad for Option<T> {
    type Item = T;
    type Wrapped<B> = Option<B>;

    fn pure(x: T) -> Self {
        Some(x)
    }

    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: FnOnce(T) -> Option<B>,
    {
        self.and_then(f)
    }
}

pub fn monad_example() -> Option<i32> {
    let x = Option::<i32>::pure(10);
    x.bind(|n| Some(n + 5))
     .bind(|n| if n > 12 { Some(n * 2) } else { None })
}

// Function currying
// Easiest way to describe what currying is, is basically 
// like it's a function with a little bit of memory. In a 
// normal function you'd use something like two parameters
// to create the final output at one instance in time. 
// 
// With a "curried" function, you can break it out into steps,
// such that you could do a part of a calculation at one point, 
// then another part of the calculation after something else happens
// in between to get the result you'd want if there is a timing constraint
// or something you need to happen outside of the function to complete the 
// full calculation. 
//
// How this works is that the curried function only calculates using one 
// parameter instead of the originall 2 paraemeters, and returns a current 
// "bookmark" or "snapshot" so to speak for the current function calculation
// by way of return the current calc + the function that contains the previous
// result so that when it is called upon again at a letter point with the 2nd
// parameter input, it completes the calculation. 
// 
// Another way to think about is like taking a parameter a step at a time
// instead of all at once. 

// Two things at once.
pub fn not_curried(p1: u32, p2: u32) -> u32 {
    p1 + p2
}

// Two steps 
pub fn curried(p1: u32) -> Box<dyn Fn(u32) -> u32> {
    Box::new(move |p2: u32| {
        p1 + p2
    })
}

// Lazy evaluation
// Basically an operation that won't do anything 
// until accessed. Or an expression that doesn't express
// until triggered to do so. It needs a poke to get it going. 
pub fn lazy_print() -> impl Fn() -> i32 {
    // let x = { println!("side effect"); 1 + 2};
    let y = ||{println!("side effect"); 1 + 2};
    y
}

/* Metaprogramming */