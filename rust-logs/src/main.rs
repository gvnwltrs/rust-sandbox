// log file: logs.txt
// size: 1036 bytes
// size on disk: 4KB

use std::io::Error;

mod file_string;
use file_string::file::read_to_stack_string;

fn check_errors(logs: &str) -> Result<&str, Error> {
    let x = "";
    for line in logs.lines() {
        if line.contains("ERROR") {
            println!("Found ERROR: {:#?}", line);
        }
    }
    Ok(x)
}

fn main() {
    const SIZE: usize = 2048;

    println!("Starting to read logs.txt");
    let logs = read_to_stack_string::<SIZE>("logs.txt")
        .expect("Failed to read logs.txt");
    println!("Finished reading logs.txt");
    println!("Size: {:#?}B, Content: {:#?}", std::mem::size_of_val(&logs), logs);
    println!("Stack string result: {:#?}", logs);

    check_errors(logs.as_str()).expect("Failed to check errors");
}
