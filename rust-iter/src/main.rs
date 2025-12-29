

fn print_elements(elements: &Vec<String>) {
    let mut iter = elements.iter();
    loop {
        match iter.next() {
            Some(cur) => println!("{:#?}", cur),
            None => break, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {

    }
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements(&colors);
}
