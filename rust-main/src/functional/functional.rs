
pub fn square<T>(x: T) -> T 
where T: std::ops::Mul<Output=T> + Copy
{  
    x*x
}

pub fn tupled(x: i32, y: f64, z: String) -> (i32, f64, String) {
    type Tuple1 = (i32, f64, String);

    let this: Tuple1 = (x, y, z); 
    println!("output: {:#?}", this);    
    this
}

pub fn get_one(collection: &Vec<String>) -> Option<&String> {
    match collection.get(0) {
        Some(x) => Some(x),
        _ => None
    } 
}

pub fn get_one_char<'a>(collection: &'a Vec<String>, c: usize) -> &'a str {
    let s = &collection[c];
    &s[0..1]
}