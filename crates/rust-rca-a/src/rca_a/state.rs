#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */

/*******************************************************************************
 * (1) States 
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

/*******************************************************************************
 * (2) Events 
******************************************************************************/

pub enum Events {
    ConnectionAccepted,
    RequestReceived(String),
    ResponseReady(String),
}