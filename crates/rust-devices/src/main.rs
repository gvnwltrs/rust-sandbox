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
    let state_msg = format!("Current state: "); 
    let mut hardware = FakeHardware;
    let mut device = new_device();
    println!("Device created: {:#?}\n", device);

    println!("Device transitioning to power on.");
    let _step = step(&mut device, &mut hardware, Some(Command::PowerOn));
    match _step { 
        Ok(result) => println!("Power on result: {:#?}", result),
        Err(e) => println!("Error: {:#?}", e)
    }
    println!("{} {:#?}\n", state_msg, device);

    println!("Device transitioning to idle.");
    let _step = step(&mut device, &mut hardware, None);
    match _step {
        Ok(result) => println!("To idle result: {:#?}", result),
        Err(e) => println!("Error: {:#?}", e)
    }
    println!("{} {:#?}\n", state_msg, device);

    Ok(())
}