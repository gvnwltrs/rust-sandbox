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
use rust_notepad::display::*;
#[allow(unused)]
use eframe::egui;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> Result<(), eframe::Error> {
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default()
    //         .with_title("rust-notepad")
    //         .with_inner_size([800.0, 600.0]),
    //     ..Default::default()
    // };

    // eframe::run_native(
    //     "rust-notepad",
    //     options,
    //     Box::new(|_cc| Ok(Box::new(NotepadApp::new()))),
    // )

    // let (tx, rx) = mpsc::channel();
    let (display_tx, display_rx) = mpsc::channel::<DisplayModel>();
    let (input_tx, input_rx) = mpsc::channel::<GuiInput>();

     thread::spawn(move || {
        run_rca_engine(display_tx, input_rx);
    });

    start_gui(display_rx, input_tx);

    Ok(())

}

// fn run_rca_engine(display_tx: mpsc::Sender<DisplayModel>, input_rx: mpsc::Receiver<GuiInput>) {
//     let mut ctx = Data::give_system_init();

//     let mut thread = ProgramThread::Main {
//         counter: 0,
//         tasks: build_tasks(),
//         handoff: CellData::None,
//     };

//     ctx.state = State::Running;

//     loop {
//         thread.step(&mut ctx).ok();

//         if let Some(display) = ctx.display_io.clone() {
//             tx.send(display).ok();
//         }

//         std::thread::sleep(std::time::Duration::from_millis(16));
//     }
// }

fn run_rca_engine(
    display_tx: Sender<DisplayModel>,
    input_rx: Receiver<GuiInput>,
) {
    let mut ctx = Data::give_system_init();

    let mut thread = ProgramThread::Main {
        counter: 0,
        tasks: build_tasks(),
        handoff: CellData::None,
    };

    ctx.state = State::Running;

    loop {
        while let Ok(input) = input_rx.try_recv() {
            match input {
                GuiInput::SetBody(text) => {
                    if let Some(display) = ctx.display_io.as_mut() {
                        display.body = text;
                    }
                }
                GuiInput::ClearRequested => {
                    if let Some(display) = ctx.display_io.as_mut() {
                        display.body.clear();
                        display.status = "status:\nCleared".to_string();
                    }
                }
                GuiInput::SaveRequested => {
                    if let Some(display) = ctx.display_io.as_mut() {
                        display.status = "status:\nSave requested".to_string();
                    }
                }
                GuiInput::OpenRequested => {
                    if let Some(display) = ctx.display_io.as_mut() {
                        display.status = "status:\nOpen requested".to_string();
                    }
                }
            }
        }

        thread.step(&mut ctx).ok();

        if let Some(display) = ctx.display_io.clone() {
            let _ = display_tx.send(display);
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn build_tasks() -> [Cell; TASK_BUFFER] {
    [ 
        Cell { id: 0, task: TaskType::DisplayData },
        Cell { id: 1, task: TaskType::CheckPerformance },
    ]
}

// fn start_gui(display_rx: Receiver<GuiInput>, display_tx: mpsc::Sender<GuiInput>) {

//     let options = eframe::NativeOptions::default();

//     eframe::run_native(
//         "rust-notepad",
//         options,
//         Box::new(|_cc| Ok(Box::new(NotepadApp::new(display_rx))))
//     ).unwrap();
// }

fn start_gui(
    display_rx: Receiver<DisplayModel>,
    input_tx: Sender<GuiInput>,
) {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "rust-notepad",
        options,
        Box::new(|_cc| Ok(Box::new(NotepadApp::new(display_rx, input_tx)))),
    )
    .unwrap();
}
