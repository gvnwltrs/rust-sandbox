use std::io;

// Why does it print 3.4028237 and not 3.4028236? 
pub fn three_and_a_bit() {
    const THREE_AND_A_BIT: f32 = 3.4028236;
    println!("{}", THREE_AND_A_BIT);
}

// What happens here? 
// Given that stdin() reads things in as a string, it should eqquivocate to "5" no?
pub fn non_standard_input() {
    println!("what is 3+2? Type your answer and press enter.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input.");

    // BAD
    // if input == "5" { 
    //     println!("Correct!");
    // } else {
    //     println!("Incorrect.");
    // }

    // GOOD
    let _: String = match input.trim().parse() {
        Ok(n) => {
            if n == String::from("5") {
                println!("Correct!");
            } else {
                println!("Incorrect.");
            }
            n
        }
        Err(_) => String::from("Bad input.")
    };
}