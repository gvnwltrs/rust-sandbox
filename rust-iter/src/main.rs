// use std::path::Iter;


trait IteratorExt: Iterator {
    fn drain(mut self) 
    where
        Self: Sized, 
    {
        while let Some(_) = self.next() {}
    }
}

impl<I: Iterator> IteratorExt for I {}

// Instructor's version:
//
// for element in elements {
//      println!("{}", element);  
//}
//
// Version without using .next()
// elements.iter().for_each(|el| println!("{:#?}", el));
//
// Versions demonstrating iterator adapters
// let map = elements.iter().map(|item| format!("{:#?}", item));
// map.for_each(|el| println!("{:#?}", el));
//
// Using map and for_each
// elements.iter().map(|x| format!("{}", x)).for_each(|x| println!("{}", x));
//
// Using $[String] to allow for passing a slice of a string versus a Vector
// Below is what's actually happening under the hood:
fn print_elements(elements: &[String]) {
    // My version: 
    let mut iter = elements.iter();
    loop {
        match iter.next() {
            Some(cur) => println!("{:#?}", cur),
            None => break, 
        }
    }
}

// Changed mutable slice for Vector of strings
fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut()
        .for_each(|x| x.truncate(1));
}

fn to_uppercase(strings: &mut [String]) -> Vec<String> {
    strings
        .iter()
        .map(|s| s.to_uppercase())
        .collect()
}

// `.consume()` probably pointeless versus just using `.for_each()`, 
// but there seems to be some cases where it might be useful...
// IRL: use .for_each(|x| {}) instead of drain() for these cases
fn to_uppercase_in_place(strings: &mut [String]) {
    strings
        .iter_mut()
        .map(|s| *s = s.to_uppercase())
        .drain();
}

fn peek(strings: &mut [String], target: &str) -> Option<()> {
    match strings
        .iter()
        .filter(|x| x.as_str() == target)
        .map(|x| format!("found it: {}", x) )
        .for_each(|x| println!("{}", x)) {
            () => Some(()),
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_strings() {
        let mut color = vec![String::from("blue")];

        shorten_strings(&mut color);
        let expect = vec![String::from("b")];

        assert_eq!(color, expect);

    }

    #[test]
    fn test_to_uppercase() {
        let mut color = vec![String::from("blue")];

        let result = to_uppercase(&mut color);
        let expect = vec![String::from("BLUE")];

        assert_eq!(result, expect);
    }

    #[test]
    fn test_to_uppercase_in_place() {
        let mut color = vec![String::from("blue")];

        to_uppercase_in_place(&mut color);
        let expect = vec![String::from("BLUE")];

        assert_eq!(color, expect);
    }

    #[test]
    fn test_peek() {
        let mut colors = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];
        let result = peek(&mut colors, "green");

        assert!(result.is_some());
    }
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elements(&colors);

    // Change to uppercase and copy to new Vector
    let mut uppercased_colors = to_uppercase(&mut colors);
    print_elements(&uppercased_colors);

    // Shorten strings to single chars using function
    shorten_strings(&mut uppercased_colors);
    print_elements(&uppercased_colors);

    // In-place modifications
    let mut _colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elements(&_colors);

    to_uppercase_in_place(&mut _colors);
    print_elements(&_colors);

    peek(&mut _colors, "GREEN");


}
