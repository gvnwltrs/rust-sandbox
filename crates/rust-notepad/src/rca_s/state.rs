#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
#[allow(unused)]
use eframe::egui;
#[allow(unused)]
use crate::display::DisplayModel;

/*******************************************************************************
 * 2. States 
******************************************************************************/

/* Status: FREEZE */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum State {
    Init,       // (0)
    Halt,       // (1)
    Running,    // (2) 
    Failure,    // (3)
    Degraded,   // (4)
    Shutdown,   // (5)
}