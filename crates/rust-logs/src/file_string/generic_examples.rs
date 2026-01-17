// Examples of calling functions with const generics

use heapless::String;
use std::io::{Error, Result};

// Your function signature
pub fn read_to_stack_string<const N: usize>(path: &str) -> io::Result<String<N>> {
    // ... implementation
}

// Different ways to call it:

pub fn example1() {
    // ✅ Turbofish syntax (what you're using) - ALWAYS works
    const SIZE: usize = 2048;
    let logs = read_to_stack_string::<SIZE>("logs.txt");
    // or with literal:
    let logs = read_to_stack_string::<2048>("logs.txt");
}

pub fn example2() {
    // ✅ Sometimes you can infer from the variable type
    // But this ONLY works if the compiler can unambiguously determine N
    let logs: Result<String<2048>, _> = read_to_stack_string("logs.txt");
    // However, this is often not possible because Result<_, _> is ambiguous
}

pub fn example3() {
    // ✅ Using a const generic in a struct/type helps inference
    struct LogBuffer<const N: usize> {
        data: String<N>,
    }
    
    // This might work in some cases, but still usually needs turbofish
    let buffer = LogBuffer {
        data: read_to_stack_string::<2048>("logs.txt")?,
    };
}

// Key points:
// 1. Type generics (like <T>) can often be inferred
// 2. Const generics (like <const N: usize>) USUALLY need turbofish
// 3. The turbofish syntax ::<...> is called that because ::<> looks like a fish
// 4. For const generics, you can use:
//    - Literal: ::<2048>
//    - Const: ::<SIZE> (where SIZE is a const)
//    - Const expr: ::<{ 1024 * 2 }>

