use std::io::Error;

fn string_test(a: String, b: &String, c: &str) -> Result<(String, String, String), Error> {
    // heap allocated strings
    let a = std::any::type_name::<String>();

    let b = std::any::type_name::<&String>();

    // stack allocated strings
    let c = std::any::type_name::<&str>();

    Ok((a.into(), b.into(), c.into()))
}

fn main() {
    println!("Now running string_test...");

    match string_test(
        String::from("Hello, world!"), 
        &String::from("Hello, world!"), 
        "Hello, world!"
    ) {
        Ok((a, b, c)) => {
            println!("a: {:#?}", a);
            println!("b: {:#?}", b);
            println!("c: {:#?}", c);
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }

    println!("NOTE: For anything that contains allocated memory, this is a heap allocated string.");
}
