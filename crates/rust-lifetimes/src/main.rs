use rust_lifetimes::*;

fn main() {
    println!("Starting language handler...");

    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let mut language = match languages.get(0) {
        Some(s) => s,
        None => "NONE",
    };

    println!("Current language: {:#?}", language);
    language = match next_language(&languages, &language) {
        Some(s) => s,
        None => "NONE",
    };
    println!("Next language: {:#?}", language);

    println!("Current language: {:#?}", language);
    language = match next_language(&languages, &language) {
        Some(s) => s,
        None => "NONE",
    };
    println!("Next language: {:#?}", language);

    println!("Last language: {:#?}", last_language(&languages)); 

    println!("Longest language: {:#?}", longest_language(&languages));
}
