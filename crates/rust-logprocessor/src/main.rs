/*******************************************************************************
 * Rust Regulated Cell Architecture (RCA) 
 * Author: Gavin Walters
 * Created: 2026-03-06
 * 
 * Description: 
 * Linear Sequential Runtime System 
 * 
 * 
 * Workflow:
 * Data -> States -> Cells -> Threads -> Engine
 * 
******************************************************************************/

// Data
#[allow(unused)]
use std::marker::PhantomData;

// Errors
#[allow(unused)]
use std::io::{ Error, Read, Write };

// Timing & performance
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
use rust_logprocessor::rca_e::*;

// Dependencies

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> std::io::Result<()> {
    let mut engine = Engine::give_default();

    let inputs = [
        "INFO Boot complete",
        "WARN Temperature rising",
        "ERROR Sensor timeout",
        "INFO Retry started",
        "ERROR Sensor timeout",
        "TRACE Something odd happened",
    ];

    for line in inputs {
        let out = engine.access_line(line)?;
        println!("{out}\n");
    }

    Ok(())
}