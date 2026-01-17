
// Demonstrating Error Handling approaches in Rust using std lib 
// "Result" struct + generics 
use std::io::Error;

pub fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}

pub fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        // Success!
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

pub fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("ingredients exceeds maximum quantity!"))
    } else {
        Ok(())
    }
}