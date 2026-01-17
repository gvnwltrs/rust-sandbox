use rust_errors::{divide, validate_email, validate_ingredients};

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
