// Demonstration of different size measurements

use std::fs;
use std::io;

/// Get file size before reading (useful for determining if SIZE is large enough)
pub fn get_file_size(path: &str) -> io::Result<usize> {
    Ok(fs::metadata(path)?.len() as usize)
}

/// Example showing all the different "sizes" you might care about
pub fn demonstrate_sizes<const N: usize>(path: &str) -> io::Result<()> {
    use heapless::String;
    use file_string::file::read_to_stack_string;
    
    // 1. File size on disk
    let file_size = get_file_size(path)?;
    println!("1. File size on disk: {} bytes", file_size);
    
    // 2. Read the file
    let logs = read_to_stack_string::<N>(path)?;
    
    // 3. String length (actual content)
    println!("2. String length (content): {} bytes", logs.len());
    
    // 4. Buffer capacity (your SIZE constant)
    println!("3. Buffer capacity (SIZE): {} bytes", logs.capacity());
    
    // 5. Struct size in memory
    println!("4. Struct size in memory: {} bytes", std::mem::size_of_val(&logs));
    println!("   (This includes the struct overhead + buffer)");
    
    // 6. Check if you have headroom
    let headroom = logs.capacity() - logs.len();
    println!("5. Headroom remaining: {} bytes", headroom);
    
    if file_size > logs.capacity() {
        println!("⚠️  WARNING: File is larger than buffer capacity!");
    } else if file_size == logs.capacity() {
        println!("⚠️  WARNING: File exactly matches capacity (no headroom)");
    } else {
        println!("✅ File fits with {} bytes headroom", headroom);
    }
    
    Ok(())
}

/// Better way to determine SIZE: check file first, then decide
pub fn determine_optimal_size(path: &str) -> io::Result<usize> {
    let file_size = get_file_size(path)?;
    
    // Add 10% headroom for safety
    let optimal = file_size + (file_size / 10);
    
    // Round up to next power of 2 (common practice)
    let rounded = optimal.next_power_of_two();
    
    println!("File size: {} bytes", file_size);
    println!("With 10% headroom: {} bytes", optimal);
    println!("Rounded to power of 2: {} bytes", rounded);
    
    Ok(rounded)
}

// However, note: You can't use this dynamically with const generics!
// You'd still need to pick a fixed size at compile time.
// This is just for guidance when choosing your SIZE constant.


