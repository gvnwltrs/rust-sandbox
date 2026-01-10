use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

#[allow(unused_imports)]
use rust_functional::functional::*;

#[allow(unused)]
enum SelectLib {
    Functional,
}

fn main() -> Result<(), Error> {
    println!("Rust Main Starting...");

    let selection = SelectLib::Functional;

    match selection {
        SelectLib::Functional => {
            // Map a squared result to 0 through 10 
            let product= square::<i32>(2);
            println!("squared result: {}", product);

            // Use closure to return mapped execution
            let x: Vec<_> = (0..10).map(|x| x*x).collect();
            println!("x: {:#?}", x);

            // Playing with tuples and type aliases
            let tup = tupled(1, 2., "".to_string());
            let _ = tup;

            // Get a char
            let letters = vec![String::from("abc")];
            println!("Collection: {:#?}", letters);
            let ch = get_one_char(&letters, 0);
            println!("Char: {:#?}", ch);

            // Demonstrating OOP 
            let my_object = MyObject::new(1, 1., String::from("hi"));

            // Demonstrating OOP + functional 1st class function

            // Still "clean" since API is not muddied up by prints which 
            // would make a lot of functions impure or side-effects.
            my_object.apply(|a, b, c| {
                println!("apply: {}, {}, {}", a,b,c);
            });

            my_object.apply(|a, b, c| {
                let temp = a + 1;
                println!("adding one: {}, {}, {}", 
                    temp,
                    b,
                    c
                );
            });

            // Using the first-class function call with a closure for a 
            // print, helps us adhere to a principle of keeping functions 
            // or operations from becoming "side-effects" by using prints 
            // in their function blocks. Ideally, we use prints only in main
            // and/or in our tests to gain insight to what happened, rather
            // than littering our production code with prints. 
            my_object.apply(|a, b, c| {
                let temp = b + 2.14;
                println!("changing to pi: {}, {}, {}", 
                    a,
                    temp,
                    c
                );
            });

            // Using a pure function (no print or external use)
            let x = pure_function1(2);
            my_object.execute_one(|a| {
                println!("Introspection: {}", a);
            }, x);

            // Using an impure function (uses print)
            impure_function(2);

            // Using immutable & mutable operations to produce tuple
            let y = immutable_operation1(); // prints tuple (i32, i32)
            my_object.execute_one(|a| {
                println!("Introspection: {:#?}", a);
            }, y);

            // Function uses functional composition to calculate result
            let z = functional_composition(100.);
            my_object.execute_one(|a| {
                println!("Introspection: {:#?}", a);
            }, z);

            // Using higher-order function
            let w = is(|x| { x > 0 }, 4); // Is x gt 0 given "4"
            my_object.execute_one(|a| {
                println!("Introspection: {:#?}", a);
            }, w);

            // Using a functor
            let q = functor();
            my_object.execute_one(|a| {
                println!("Introspection: {:#?}", a);
            }, q);

            // Mondad example
            let m = monad_example();
            my_object.execute_one(|a| {
                println!("Introspection: {:#?}", a);
            }, m);

            // Function currying
            let a = not_curried(2, 2);
            my_object.execute_one(|a| {
                println!("Introspection: {:#?}", a);
            }, a);

            let b = curried(2);
            let c = b(2);
            my_object.execute_one(|a| {
                println!("Introspection: {:#?}", a);
            }, c);

            // Lazy evaluation print 
            let l_print = lazy_print();
            l_print(); // doesn't print until "kicked" or "poked"

            // Metaprogramming 
            print();
            macro_rule_exe(); 
            macro_branching();

            // NOTE: Functional control flow -- see -> rust-simulator
        }
    }

    Ok(())
}
