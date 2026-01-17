
// Not a pure function - common
pub fn my_func() { println!("this works");}

// This is a "pure function" because it does not modify the original, or
// the input "x: T". It only use a read-only of "x" to perform a calculation
// and returns the result of that calculation which is then owned by the caller.
pub fn square<T>(x: T) -> T 
where T: std::ops::Mul<Output=T> + Copy {  
    x*x
}

// This is NOT a "pure function". This is "impure" or a "side-effect"
// because it uses println! which is an IO call to the external system
// or OS.
pub fn tupled(x: i32, y: f64, z: String) -> (i32, f64, String) {
    type Tuple1 = (i32, f64, String);

    let this: Tuple1 = (x, y, z); 
    println!("output: {:#?}", this);    
    this
}

// This is still a "pure function" because we only return a ref
// that points backto the source of the ref. Nothing is modified 
// in the original.
pub fn get_one(collection: &Vec<String>) -> Option<&String> {
    match collection.get(0) {
        Some(x) => Some(x),
        _ => None
    } 
}

// This is a "pure function" because it only reads then points to a specific segment 
// of the original. In this case we only look-at/read the ref to select
// a section to return as a ref which points to a slice of the original. 
pub fn get_one_char<'a>(collection: &'a Vec<String>, c: usize) -> &'a str {
    let s = &collection[c];
    &s[0..1]
}