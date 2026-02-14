/* rust-main::lib.rs */

#[allow(unused)]
use core::fmt::Write;

#[allow(unused)]
use std::io::Error;

#[allow(unused)]
// use core::fmt::Result;

#[allow(unused)]
use chrono::{Local, Utc};

/* 1. Data  */

#[derive(Debug, Clone, PartialEq)]
pub enum Units {
	Fahrenheit,
	Celsius,
	Kelvins,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThermostatState {
    Off,
    Booting,
    Idle {
        temperature: f64,
        setpoint: f64,
    },
    SettingSetpoint {
        temperature: f64,
        target: f64,
    },
    Fault,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Thermostat {
    pub state: ThermostatState,
    pub units: Units, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PowerOn,
    SetSetpoint(f64),
    Shutdown,
}

pub trait ThermostatHardware {
    fn give_temperature(&self) -> f64;
    fn mutate_setpoint(&mut self, _temp: f64) -> bool;
} 

pub struct FakeHardware;

impl ThermostatHardware for FakeHardware {
    fn give_temperature(&self) -> f64 {
        65.0
    }

    fn mutate_setpoint(&mut self, _temp: f64) -> bool {
        true
    }
}

/* 2. Actions */

#[allow(unused)]
fn give_timestamp() -> Option<String> {
    Some(Local::now().format("%Y-%m-%dT%H:%M").to_string())
}

pub fn mutate_state(
    device: &mut Thermostat,
    hw: &mut impl ThermostatHardware,
    cmd: Option<Command>,
) -> Result<(), ()> {
    device.state = match (&device.state, cmd) {

        (ThermostatState::Off, Some(Command::PowerOn)) => {
            ThermostatState::Booting
        }

        (ThermostatState::Booting, None) => {
            let temp = hw.give_temperature();
            ThermostatState::Idle {
                temperature: temp,
                setpoint: 68.0,
            }
        }

        (
            ThermostatState::Idle { temperature, .. },
            Some(Command::SetSetpoint(target)),
        ) => {
            ThermostatState::SettingSetpoint {
                temperature: *temperature,
                target,
            }
        }

        (
            ThermostatState::SettingSetpoint { temperature, target },
            None,
        ) => {
            if hw.mutate_setpoint(*target) {
                ThermostatState::Idle {
                    temperature: *temperature,
                    setpoint: *target,
                }
            } else {
                ThermostatState::Fault
            }
        }

        (_, Some(Command::Shutdown)) => ThermostatState::Off,

        (state, _) => state.clone(), // ignore invalid commands safely
    };

    Ok(())
}

/* 3. Pure Functions */

pub fn give_device() -> Thermostat {
    Thermostat {
        state: ThermostatState::Off,
        units: Units::Fahrenheit,
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
    fn test_thermostat_boot() {
        let mut hw = FakeHardware;
        let mut dev = give_device();
        mutate_state(&mut dev, &mut hw, Some(Command::PowerOn)).unwrap();
        mutate_state(&mut dev, &mut hw, None).unwrap();
        assert!(matches!(dev.state, ThermostatState::Idle { .. }));
    }
	
    // 2. Set operation
    #[test]
    fn test_thermostat_setpoint() {
        let mut hw = FakeHardware;
        let mut dev = give_device();
        mutate_state(&mut dev, &mut hw, Some(Command::PowerOn)).unwrap();
        mutate_state(&mut dev, &mut hw, None).unwrap();
        mutate_state(&mut dev, &mut hw, Some(Command::SetSetpoint(72.0))).unwrap();
        mutate_state(&mut dev, &mut hw, None).unwrap();

        match dev.state {
            ThermostatState::Idle { setpoint, .. } => {
                assert_eq!(setpoint, 72.0);
            }
            _ => panic!("unexpected state"),
        }

    }

    #[test]
    fn test_stub() {
    }

}