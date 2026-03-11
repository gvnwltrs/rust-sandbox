/*******************************************************************************
 * Rust Regulated Cell Architecture (RCA) 
 * Author: Gavin Walters
 * Created: 2026-03-06
 * 
 * Description: 
 * RCA-S Simulation Engine Experiment
 * 
 * 
 * Workflow:
 * Data -> Constraints -> Cells -> Threads -> Engine
 * 
******************************************************************************/

// Data
#[allow(unused)]
use std::marker::PhantomData;
#[allow(unused)]
use std::io::Error;
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
use rust_simulationengine::rca_s::*;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> std::io::Result<()> {
    let mut engine = Engine::give_default();
    engine.access_run(5)?;
    Ok(())
}