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
    let mut device: (ThermostatDataPoint, Status)= gen_thermo_instance();
    let mut device = init_device(&mut device.0);
    println!("Device initialized: {:#?}", device);
    let status = check_status(&device.0);
    println!("Device status: {:#?}", status);
    println!("Device state: {:#?}\n", device);

    println!("Device temp setpoint being modified...");
    let temp_setting = ThermostatEvent::Setpoint(67.0);
    let device: (ThermostatDataPoint, Status) = set_operation(&mut device.0, &temp_setting);
    println!("Device configuration: {:#?}", device.1);
    println!("Device updated: {:#?}\n", device);

    Ok(())
}


pub type EmptyString<'a> = &'a str;
#[allow(unused)]
const EMPTY_STR: EmptyString = "";