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

#[allow(unused)]
use std::marker::PhantomData;
#[allow(unused)]
use std::time::Instant;
#[allow(unused)]
use rust_actormessagesystem::rca_e::*;

/* Dependencies */

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> std::io::Result<()> {
    let mut engine = <Engine as PrimaryRunner>::give_default();
    engine.try_run_engine()?;
    Ok(())
}