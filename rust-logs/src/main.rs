use std::fs;

mod file_string;
use file_string::file::FileBuf;

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

fn main() -> Result<(), std::io::Error> {
    // Stack-based log
    const SIZE: usize = 2048;
    let mut file_buf = FileBuf::<SIZE>::new();

    file_buf.read_to_buf("logs.txt")?;

    
    let full_log = file_buf.extract_all()?;
    println!("Log: {:#?}", full_log);

    let errors = file_buf.extract_errors()?;
    let warnings = file_buf.extract_warnings()?;

    println!("Errors: {:#?}", errors);   
    println!("Warnings: {:#?}", warnings);

    // Compare to heap-based log
    let heap_log = fs::read_to_string("logs.txt")?;
    let heap_errors = extract_errors(&heap_log);
    println!("Heap errors: {:#?}", heap_errors);

    Ok(())
}
