
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
    #[allow(unused)]
    let immutable_v1 = 1;
    // immutable_v1 = 2; //invalid

    #[allow(unused)]
    let mut mutable_v2 = 1;
    mutable_v2 = 2;

    let output = (immutable_v1, mutable_v2);
    output
}

// Functional composition:
// Uses output of one function as input to another function
// by being connected. 
pub fn functional_composition(x: f64) -> f64{
    #[allow(unused, nonstandard_style)]
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
        F: FnOnce(T) -> Option<B>, {
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

// Types:
// 1. recusive
// 2. procedural
//
// Both macro types take abstract syntax tree (AST) as input and produce one or more AST's. 

// Example metaprogramming application: `println!()` macro
pub fn print() {
    println!("This is a metaprogramming implementation: `println!()` Rust macro.");
}

// Other metaprogramming macros:
// * vec![]
// * macro_rules!
// Example:
//
// Expects and matches only a comma-separated list of expressions. ( $( $x: expr ),* ) matches against a 
// comma separaed list of expressions and stores the result in the plural variable `$x`. The single 
// block in the body defines a new `vec`, then iterates through `$x` to push each `$x` into the vec,
// and finally the block returns the vec as its result. 
pub fn macro_rule_exe() {
    macro_rules! my_vec_macro {
        { $( $x:expr ),* } => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        }
    }
    my_vec_macro!(1, 2, 3); // using the macro 
}

// "Recursive" macro 
pub fn macro_branching() {
    macro_rules! my_macro_branch {
        (1 $e:expr) => (println!("mode 1: {}", $e));
        (2 $e:expr) => (println!("mode 2: {}", $e));
    }

    // using my_macro_branch
    my_macro_branch!(1 "abc");
    my_macro_branch!(2 "def");
}