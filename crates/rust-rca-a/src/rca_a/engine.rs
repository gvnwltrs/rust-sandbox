#[allow(unused)]
use crate::rca_a::{ Data, DisplayModel, GuiInput, ProgramThread, TaskType, Cell, CellData, State };

use eframe::*;
use std::sync::mpsc::{Receiver, Sender};
#[allow(unused)]
use std::thread;
#[allow(unused)]
use std::sync::mpsc;

/*******************************************************************************
 * (1) Default 
******************************************************************************/

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/

pub fn run_rca_engine(
    display_tx: Sender<DisplayModel>,
    input_rx: Receiver<GuiInput>,
) {
    let mut ctx = Data::give_system_init();

    let mut thread = ProgramThread::build_tasks(
        Some(0),
        Some([ 
            Cell { id: 0, task: TaskType::None }, 
            Cell { id: 1, task: TaskType::None }, 
        ]),
        None,
    ); 

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

pub fn start_gui(
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


pub struct NotepadApp {
    pub display_rx: Receiver<DisplayModel>,
    pub input_tx: Sender<GuiInput>,
    pub display: Option<DisplayModel>,
}

impl NotepadApp {
    pub fn new(
        display_rx: Receiver<DisplayModel>,
        input_tx: Sender<GuiInput>,
    ) -> Self {
        Self {
            display_rx,
            input_tx,
            display: None,
        }
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(display) = self.display_rx.try_recv() {
            self.display = Some(display);
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        let _ = self.input_tx.send(GuiInput::OpenRequested);
                        ui.close();
                    }

                    if ui.button("Save").clicked() {
                        let _ = self.input_tx.send(GuiInput::SaveRequested);
                        ui.close();
                    }

                    if ui.button("Clear").clicked() {
                        let _ = self.input_tx.send(GuiInput::ClearRequested);
                        ui.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(display) = self.display.as_mut() {
                ui.heading(&display.title);
                ui.separator();

                let response = ui.add(
                    egui::TextEdit::multiline(&mut display.body)
                        .desired_rows(20)
                        .desired_width(f32::INFINITY),
                );

                if response.changed() {
                    let _ = self
                        .input_tx
                        .send(GuiInput::SetBody(display.body.clone()));
                }

                ui.separator();
                ui.label(&display.status);
            } else {
                ui.heading("rust-notepad");
                ui.label("Waiting for engine...");
            }
        });

        ctx.request_repaint();
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;

    #[test]
    fn smoke_test() {
        assert!(true);
    }
} 