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
 * Data -> Constraints -> Cells -> Threads -> Engine
 * 
******************************************************************************/

// Data
#[allow(unused)]
use std::marker::PhantomData;
use std::io::Error;
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
use rust_rca::rca::*;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> Result<(), Error> {
    let mut engine = <Engine as PrimaryRunner>::give_default();
    engine.try_run_engine()?;
    Ok(())
}