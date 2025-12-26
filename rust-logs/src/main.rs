use std::fs;

fn main() {
    // TODO: reading into a heap-based String; change to stack
    match fs::read_to_string("logs.txt")  {
        Ok(contents) => println!("SUCCESS: file read -- {:#?}", contents), 
        Err(failure) => {
            println!("ERROR: {:#?}", failure); 
        } 
    }

    match fs::read_to_string("logs2.txt")  {
        Ok(contents) => println!("SUCCESS: file read -- {:#?}", contents), 
        Err(failure) => {
            println!("ERROR: {:#?}", failure); 
        } 
    }
}
