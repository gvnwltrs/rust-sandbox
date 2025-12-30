

fn print_elements(elements: &Vec<String>) {

    // Instructor's version:
    //
    // for element in elements {
    //      println!("{}", element);  
    //}
    //
    // Below is what's actually happening under the hood:

    // My version: 
    // let mut iter = elements.iter();
    // loop {
    //     match iter.next() {
    //         Some(cur) => println!("{:#?}", cur),
    //         None => break, 
    //     }
    // }

    // Version without using .next()
    // elements.iter().for_each(|el| println!("{:#?}", el));

    // Versions demonstrating iterator adapters
    // let map = elements.iter().map(|item| format!("{:#?}", item));
    // map.for_each(|el| println!("{:#?}", el));

    // Using map and for_each
    elements.iter().map(|x| format!("{}", x)).for_each(|x| println!("{}", x));

    // Using filter and for_each
    elements.iter().filter(|x| *x == "green").for_each(|x| println!("{}", x));

    // Using map, filter, and for_each
    elements
        .iter()
        .filter(|x| x.as_str() == "green")
        .map(|x| format!("found it: {}", x) )
        .for_each(|x| println!("{}", x));

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
