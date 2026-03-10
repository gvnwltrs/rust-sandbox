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
 * Data -> States -> Threads -> Cells -> Engine
 * 
******************************************************************************/

// Data
#[allow(unused)]
use std::marker::PhantomData;

// Errors
#[allow(unused)]
use std::io::Error;

// Timing & performance
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
use rust_rca_a::rca_a::*;

// Dependencies
use eframe::*;
use std::thread;
#[allow(unused)]
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> Result<(), eframe::Error> {
    let (display_tx, display_rx) = mpsc::channel::<DisplayModel>();
    let (input_tx, input_rx) = mpsc::channel::<GuiInput>();

     thread::spawn(move || {
        run_rca_engine(display_tx, input_rx);
    });

    start_gui(display_rx, input_tx);

    Ok(())

}