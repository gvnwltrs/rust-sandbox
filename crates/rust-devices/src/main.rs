/*
    Rust Devices 
    Date created: 2026-02-07
    Date modified: 2026-02-07
*/
use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

#[allow(unused)]
use rust_devices::*;

fn main() -> Result<(), Error> {
    let device = new_device();
    println!("Device created: {:#?}", device);

    Ok(())
}