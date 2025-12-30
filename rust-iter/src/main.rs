
// Using $[String] to allow for passing a slice of a string versus a Vector
fn print_elements(elements: &[String]) {

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

fn shorten_strings(elements: &mut Vec<String>) {
    elements.iter_mut()
        .for_each(|x| x.truncate(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_strings() {
        let mut color = vec![String::from("blue")];
        let expect = vec![String::from("b")];
        shorten_strings(&mut color);

        assert_eq!(color, expect);

    }
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements(&colors[0..3]);

    let mut color = String::from("blue");
    println!("{}", color);

    color.truncate(1); // slicing to first letter
    println!("{}", color);

    shorten_strings(&mut colors);
    print_elements(&colors[..]);
}
