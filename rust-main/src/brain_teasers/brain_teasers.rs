use std::io;
pub enum Select {
    BadVersion,
    GoodVersion,
}

// Why does it print 3.4028237 and not 3.4028236? 
pub fn three_and_a_bit() {
    const THREE_AND_A_BIT: f32 = 3.4028236;
    println!("{}", THREE_AND_A_BIT);
}

// What happens here? 
// Given that stdin() reads things in as a string, it should eqquivocate to "5" no?
pub fn non_standard_input(selection: Select) {
    println!("what is 3+2? Type your answer and press enter.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input.");

    match selection {
        Select::BadVersion => {
            if input == "5" { println!("Correct!"); } 
            else { println!("Incorrect."); }
        }
        Select::GoodVersion => {
            match input.trim() {
                "5" => println!("Correct!"),
                _   => println!("Incorrect.")
            }
        }
    };

}
// Since ".trim()" returns a string, we can just use it as is, which is to
// say, using the string that results from .trim() by default (&str).
// The .trim() method removes white spaces and escape chars like "\n".
// In other scenarios, say where we need to get an actual number and
// not a string, we'd use ".parse()" to parse the input then .unwrap() to 
// get the number or convert the string to a number. Could also use 
// .expect() after parse instead of .unwrap(). 
// 
// What happened here before with the "BadVersion" was that the input was 
// still in raw string form, meaning it had the extra new line char "\n"
// still in it, so when running the if-else, it would always evaluate to 
// else since the condition was actually "5\n" == "5" before using .trim().

// Type conversion
pub fn type_conversion() {
    let x: u64 = 4_294_967_296;
    println!("x: {} ", x);
    let y: u32 = x as u32; 
    println!("y: {} ", y);

    if x == y as u64 {
        println!("x equals y.");
    } else {
        println!("x does not equal y.");
    }

}