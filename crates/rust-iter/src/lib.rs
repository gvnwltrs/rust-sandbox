// use std::path::Iter;

// NOTE: Setup iter traint (extension) to implement a "drain"
// consumer that returns nothing, but allows iter adapters to execute
pub trait IteratorExt: Iterator {
    fn drain(mut self) 
    where
        Self: Sized, 
    {
        while let Some(_) = self.next() {}
    }
}

impl<I: Iterator> IteratorExt for I {}

// Using $[String] to allow for passing a slice of a string versus a Vector
// Below is what's actually happening under the hood:
pub fn print_elements(elements: &[String]) {
    // My version: 
    let mut iter = elements.iter();
    loop {
        match iter.next() {
            Some(cur) => println!("{:#?}", cur),
            None => break, 
        }
    }
}

pub fn print_nested_elements(elements: &Vec<Vec<String>>) {
    elements.iter()
        .for_each(|inner_vec| { // pull out each inner vec
            inner_vec.iter()
                .for_each(|el| print!("{:#?},", el)); // print each element in inner vec
            println!();
    });
}

// Changed mutable slice for Vector of strings
pub fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut()
        .for_each(|x| x.truncate(1));
}

pub fn to_uppercase(strings: &mut [String]) -> Vec<String> {
    strings
        .iter()
        .map(|s| s.to_uppercase())
        .collect()
}

// `.consume()` probably pointeless versus just using `.for_each()`, 
// but there seems to be some cases where it might be useful...
// IRL: use .for_each(|x| {}) instead of drain() for these cases
pub fn to_uppercase_in_place(strings: &mut [String]) {
    strings
        .iter_mut()
        .map(|s| *s = s.to_uppercase())
        .drain();
}

pub fn to_lowercase_in_place(strings: &mut [String]) {
    strings
        .iter_mut()
        .map(|s| *s = s.to_lowercase())
        .drain();
}

pub fn peek(strings: &mut [String], target: &str) -> Option<()> {
    match strings
        .iter()
        .filter(|x| x.as_str() == target)
        .map(|x| format!("found it: {}", x) )
        .for_each(|x| println!("{}", x)) {
            () => Some(()),
        }
}

pub fn move_elements(source: Vec<String>, destination: &mut Vec<String>) {
    source
        .into_iter()
        .for_each(|s| destination.push(s))
}

// Should "exlode" string into a vec of char vectors
pub fn explode(strings: Vec<String>) -> Vec<Vec<String>> {
    strings
        .into_iter()
        .map(|s| s.chars().map(|c| c.to_string()).collect())
        .collect()
}

pub fn find_color_or(colors: &[String], search: String, fallback: String) -> Option<()> {
    colors
        .iter()
        .any(|c| c.as_str() == search || c.as_str() == fallback)
        .then_some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_strings() {
        let mut color = vec![String::from("blue")];

        shorten_strings(&mut color);
        let expected = vec![String::from("b")];

        assert_eq!(color, expected);

    }

    #[test]
    fn test_to_uppercase() {
        let mut color = vec![String::from("blue")];

        let result = to_uppercase(&mut color);
        let expected = vec![String::from("BLUE")];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_uppercase_in_place() {
        let mut color = vec![String::from("blue")];

        to_uppercase_in_place(&mut color);
        let expected = vec![String::from("BLUE")];

        assert_eq!(color, expected);
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

    #[test]
    fn test_move_elements() {
        let colors = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];
        // By not passing colors as a reference, we are essentially 
        // saying: you are gonna have a new owner now -- godspeed
        let mut copied_colors = Vec::<String>::new();
        move_elements(colors, &mut copied_colors);

        let expected = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        assert_eq!(copied_colors, expected);
    }

    #[test]
    fn test_explode() {
        let colors = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        let result = explode(colors);

        let expected = vec![
            vec![String::from("r"), String::from("e"), String::from("d")],
            vec![String::from("g"), String::from("r"), String::from("e"), String::from("e"), String::from("n")],
            vec![String::from("b"), String::from("l"), String::from("u"), String::from("e")],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_color_or() {
        let mut colors = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];
        let result = find_color_or(
            &mut colors, 
            String::from("yellow"), 
            String::from("blue")
        );

        assert!(result.is_some());
    }

}