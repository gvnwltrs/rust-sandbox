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
    let mut hardware = FakeHardware;
    let mut device = give_device();
    access_report(&device);

    let _step = mutate_state(&mut device, &mut hardware, Some(Command::PowerOn));
    access_report(&device);

    let _step = mutate_state(&mut device, &mut hardware, None);
    access_report(&device);


    Ok(())
}

// Helpers
fn access_report(device: &Thermostat) {
    match device.state {
        ThermostatState::Off => println!("Device state: Off"),
        ThermostatState::Booting => println!("Device state: Booting..."),
        ThermostatState::Idle { temperature, setpoint } => println!("Device state: Idle, temp: {:#?}, setpoint: {:#?}", &temperature, &setpoint),
        ThermostatState::SettingSetpoint { temperature, target } => println!("Device state: SettingSetpoint, temp: {:#?}, target: {:#?}", &temperature, &target),
        ThermostatState::Fault => println!("Device state: Fault"),
    }
}