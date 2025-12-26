use std::fs;
use std::io::Error;

/* 
function extract_errors(log: string) -> list of strings: 
    split log by newline characters into lines
    initialize an empty list called result

    for each line in lines:
        if line starts with "ERROR": 
            add the line to the results list

    return result list
*/
fn extract_errors(logs: &String) -> Result<Vec<String>, Error> {
    let mut logged_errors = Vec::new();
    for line in logs.lines() {
        if line.starts_with("ERROR") {
            logged_errors.push(line.into());
        }
    }
    Ok(logged_errors)
}

fn main() {
    let mut file = String::new();

    // TODO: reading into a heap-based String; change to stack
    match fs::read_to_string("logs.txt")  {
        Ok(contents) => {
            println!("SUCCESS: file read -- {:#?}", contents);
            file = contents.into();
        }
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

    match extract_errors(&file) {
        Ok(errors) => println!("SUCCESS: errors extracted -- {:#?}", errors),
        Err(failure) => println!("ERROR: {:#?}", failure),
    }
}
