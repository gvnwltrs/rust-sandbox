use rust_iter::*;

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elements(&colors);

    // Change to uppercase and copy to new Vector
    let mut uppercased_colors = to_uppercase(&mut colors);
    print_elements(&uppercased_colors);

    // Shorten strings to single chars using function
    shorten_strings(&mut uppercased_colors);
    print_elements(&uppercased_colors);

    // In-place modifications
    let mut _colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elements(&_colors);

    to_uppercase_in_place(&mut _colors);
    print_elements(&_colors);

    peek(&mut _colors, "GREEN");
    to_lowercase_in_place(&mut uppercased_colors);

    move_elements(uppercased_colors, &mut _colors);
    print_elements(&_colors);

    let exploded = explode(colors);
    print_nested_elements(&exploded);

    let color_green_found = find_color_or(&_colors, String::from("green"), String::from("GREEN"));
    println!("Color GREEN found? {:#?}", color_green_found.is_some());

    let color_red_found = find_color_or(&_colors, String::from("red"), String::from("RED"));
    println!("Color RED found? {:#?}", color_red_found.is_some());

}
