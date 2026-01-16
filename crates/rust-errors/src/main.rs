
// Demonstrating Error Handling approaches in Rust using std lib 
// "Result" struct + generics 

use std::io::Error;

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        // Success!
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("ingredients exceeds maximum quantity!"))
    } else {
        Ok(())
    }
}

fn main() {

    // Demonstrates how using Result<> assigns a return into param vars 
    match divide(5.0, 3.0) {
        Ok(result_of_division) => {
            println!("{:#?}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{:#?}", what_went_wrong);
        }
    }

    match validate_email(String::from("george.email.com")) {
        Ok(..) => println!("Email is valid"),
        Err(somethings_wrong) => {
            println!("Email format invalid: {:#?}", somethings_wrong);
        }
    }

    let ingredients = vec![
        String::from("cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"), 
        String::from("Olives"), 
    ]; 

    match validate_ingredients(&ingredients) {
        Ok(..) => println!("Ingredients qty valid"),
        Err(invalid) => {
            println!("Ingredients invalid: {:#?}", invalid);
        }
    }

}
