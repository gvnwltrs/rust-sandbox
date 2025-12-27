use std::fs;
use std::io::Error;

mod file_string;
use file_string::file::read_to_stack_string;
use file_string::file::read_to_buf;
use file_string::file::extract_string_from_buf;

/* 
function extract_errors(log: string) -> list of strings: 
    split log by newline characters into lines
    initialize an empty list called result

    for each line in lines:
        if line starts with "ERROR": 
            add the line to the results list

    return result list
*/
fn extract_errors(logs: &str) -> Vec<&str> {
    let split_text = logs.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_errors() {
        let file = fs::read_to_string("logs.txt").unwrap();
        let errors = extract_errors(&file);
        assert_eq!(errors[0], "ERROR 14:33:45 Failed to connect to the database.");
    }
}

fn main() {
    const SIZE: usize = 2048;

    // TODO: reading into a heap-based String; change to stack
    match fs::read_to_string("logs.txt") { // returns Result<String, Error>
        Ok(file) => { 
            let errors = extract_errors(&file);
            println!("Errors: {:#?}", errors);
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }

    match read_to_stack_string::<SIZE>("logs.txt") {
        Ok(file) => {
            let errors = extract_errors(&file);
            println!("Errors: {:#?}", errors);
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }

    let logs = match read_to_buf::<SIZE>("logs.txt") {
        Ok(buf) => {
            let logs = extract_string_from_buf::<SIZE>(buf, 1024);
            println!("Logs: {:#?}", logs);
            logs
        }
        Err(e) => {
            println!("Error: {:#?}", e);
            return;
        }
    };

    println!("String outside of match: {:#?}", logs);
}
