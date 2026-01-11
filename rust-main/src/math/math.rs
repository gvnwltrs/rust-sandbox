
pub fn add_two(a: i32, b: i32) -> i32 {
    a+b
}

pub fn add_some(a: i32, b: i32, sum: &mut i32) -> i32 {
    // Defensive copy
    let _a = a;
    let mut _sum = sum.clone();
    _sum = a + b;
    _sum 
}