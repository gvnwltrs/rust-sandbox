/*******************************************************************************
 * Rust Notepad 
 * Author: Gavin Walters
 * Created: 2026-03-06
 * 
 * Description: 
 * Linear Sequential Runtime System 
 * 
 * 
 * Workflow:
 * Data -> States -> Threads -> Tasks -> Engine
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
use rust_notepad::rca_s::*;
#[allow(unused)]
use rust_notepad::app::*;
#[allow(unused)]
use eframe::egui;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("rust-notepad")
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "rust-notepad",
        options,
        Box::new(|_cc| Ok(Box::new(NotepadApp::new()))),
    )
}