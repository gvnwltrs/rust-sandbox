use rust_strings::string_test;

fn main() {
    println!("Now running string_test...");

    // 1. "String::from" creates a metadata struct on the stack, then .data value is copied to the heap
    // 2. "&String::from" creates a metadata struct on the stack, then .data value is copied to the heap
    //     - a reference is created in the stack that points to the metadata struct also on stack
    // 3. "Hello, world!" is a string literal stored in the .data section (string slice) -- no heap 
    //     - still has a metadata struct on stack, but points to the string slice in the .data section
    match string_test() {
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
