// use rust_modules::module_example_1::do_something_a;
use rust_modules::module_example_1::do_something_a;
use rust_modules::module_example_2::do_something_b;

fn main() {
    println!("Hello, world!");
    println!("{:#?}", do_something_a());
    println!("{:#?}", do_something_b());
}
