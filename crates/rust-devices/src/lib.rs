/* rust-main::lib.rs */

#[allow(unused)]
use core::fmt::Write;

#[allow(unused)]
use std::io::Error;

#[allow(unused)]
use core::fmt::Result;

#[allow(unused)]
use chrono::{Local, Utc};

/* 1. Data  */

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Ready,
	Good,
	Working,
	Success,
	Error,
	Degraded,
	Warning(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThermostatEvent {
	PowerOn,
    Setpoint(f64),
	Shutdown,
	Awaiting,
	Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveEvent {
	Processing,
	Running,
	Inactive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Units {
	Farenheit,
	Celsius,
	Kelvins,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThermostatDataPoint {
	timestamp: Option<String>,
	temperature: Option<f64>,
	setpoint: Option<f64>, 
	trigger_event: ThermostatEvent,
	active_event: ActiveEvent,
	units: Units,
}

/* 2. Actions */

fn gen_timestamp() -> Option<String> {
    Some(Local::now().format("%Y-%m-%dT%H:%M").to_string())
}

fn fake_power_on_device(_sp: Option<f64>) -> (Status, f64) {
   // Fake implementation 
   let fake_reading = 65.0;
   (Status::Good, fake_reading)
}

pub fn init_device(device: &mut ThermostatDataPoint) -> (ThermostatDataPoint, Status) {
    let mut _device = device.clone();
    const DEFAULT_TEMP: f64 = 68.0;
    _device.timestamp = gen_timestamp();
    _device.trigger_event = ThermostatEvent::PowerOn;
    _device.active_event = ActiveEvent::Processing;
    let set_on = Some(DEFAULT_TEMP); 
    _device.setpoint = set_on;
    let set_on: (Status, f64) = fake_power_on_device(set_on);
    match set_on.0 {
        Status::Good => { 
            _device.trigger_event = ThermostatEvent::Awaiting;
            _device.active_event = ActiveEvent::Running;
            _device.temperature = Some(set_on.1);
            (_device, Status::Success)
        },
        _ => { 
            _device.trigger_event = ThermostatEvent::Error;
            _device.active_event = ActiveEvent::Inactive;
            (_device, Status::Error)
        }
    }
}

fn fake_set_temperature_on_device(_device: &mut ThermostatDataPoint) -> Status {
    _device.temperature = Some(65.14);
    Status::Success
}

fn modify_temp_setpoint(device: &mut ThermostatDataPoint, temp: f64) -> (ThermostatDataPoint, Status) {
    let mut _device = device.clone();
    _device.timestamp = gen_timestamp();
    match device.trigger_event {
        ThermostatEvent::Awaiting => {
            _device.trigger_event = ThermostatEvent::Setpoint(temp);
            _device.active_event = ActiveEvent::Processing;
            match fake_set_temperature_on_device(&mut _device) {
                Status::Success => {
                    _device.setpoint = Some(temp);
                    _device.trigger_event = ThermostatEvent::Awaiting;
                    _device.active_event = ActiveEvent::Running;
                    return (_device, Status::Success)
                },
                _ => return (_device, Status::Error)
            };
        },
        _ => {
            _device.trigger_event = ThermostatEvent::Error;
            _device.active_event = ActiveEvent::Running; // doesn't set, continues running
            return (_device, Status::Error)
        }
    }
}

pub fn set_operation(device: &mut ThermostatDataPoint, cfg: &ThermostatEvent) -> (ThermostatDataPoint, Status) {
    let mut _device = device.clone();
    _device.timestamp = gen_timestamp();
    // let mut _device = device.clone();
    let msg = format!("Msg: Not a configurable operation -> ");
    match cfg {
        ThermostatEvent::PowerOn => init_device(&mut _device),
        ThermostatEvent::Setpoint(temp) => { 
            modify_temp_setpoint(&mut _device, *temp)
        },
        invalid_operation => {
            let operation = format!("{:#?}", invalid_operation);
            let msg = msg + &operation[..];
            (_device, Status::Warning(msg))
        }
    }
}

/* 3. Pure Functions */

pub fn gen_thermo_instance() -> (ThermostatDataPoint, Status) {
    (
        ThermostatDataPoint {
            timestamp: gen_timestamp(),
            temperature: None,
            setpoint: None,
            trigger_event: ThermostatEvent::Awaiting,
            active_event: ActiveEvent::Inactive,
            units: Units::Farenheit, // DEFAULT
        },
        Status::Success
    )
}

pub fn check_status(device: &ThermostatDataPoint) -> Status {
    match device.trigger_event {
        ThermostatEvent::PowerOn => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Error,
                ActiveEvent::Processing => Status::Working,
                ActiveEvent::Running => Status::Error,
            }
        },
        ThermostatEvent::Setpoint(_) => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Error,
                ActiveEvent::Processing => Status::Working,
                ActiveEvent::Running => Status::Error,
            }
        },
        ThermostatEvent::Shutdown => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Success,
                ActiveEvent::Processing => Status::Working,
                ActiveEvent::Running => Status::Error,
            }
        }
        ThermostatEvent::Awaiting => { 
            match device.active_event { 
                ActiveEvent::Inactive => Status::Ready,
                ActiveEvent::Processing => Status::Error,
                ActiveEvent::Running => Status::Good,
            }
        },
        ThermostatEvent::Error => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Error,
                ActiveEvent::Processing => Status::Degraded,
                ActiveEvent::Running => Status::Warning(String::from("Msg: Running in bad state.")),
            }
        }, 
    }
}

/* 4. Tests  */

#[cfg(test)]
mod rust_device_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_sanity() {
        assert!(true);
    }

    // 1. Power on stage
    #[test]
    fn test_device_check_status_handles_defaults() {
        let thermo_instance = gen_thermo_instance(); 
        let status = check_status(&thermo_instance.0);
        let expected = Status::Ready;
        assert_eq!(status, expected);
    }

    #[test]
    fn test_boot_thermostat_power_on_good() {
        let mut thermo_instance = gen_thermo_instance(); 
        let initialized = init_device(&mut thermo_instance.0);
        let expected = Status::Success; 
        assert_eq!(initialized.1, expected);
    }
	
    // 2. Set operation
    #[test]
    fn test_configure_new_temp_setpoint() {
        let mut temp_device = gen_thermo_instance();
        let status = check_status(&temp_device.0);
        let expected = Status::Ready;
        assert_eq!(status, expected);
        
        let config = ThermostatEvent::Setpoint(68.0);
        let operation_set = set_operation(&mut temp_device.0, &config);
        let expected = Status::Success;
        assert_eq!(operation_set.1, expected);
    }

}