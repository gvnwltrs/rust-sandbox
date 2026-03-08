
use crate::rca_s::{ Data, ProgramThread };
use crate::display::{ DisplayModel, GuiInput };
use eframe::*;

use std::sync::mpsc::{Receiver, Sender};
use std::thread;

// pub struct NotepadApp {
//     pub ctx: Data,
//     pub thread: ProgramThread,
// }

// impl NotepadApp {
//     pub fn new() -> Self {
//         Self {
//             ctx: Data::give_system_init(),
//             thread: ProgramThread::Main { 
//                 counter: 0,
//                 tasks: [ 
//                     Cell { id: 0, task: TaskType::DisplayData },
//                     Cell { id: 1, task: TaskType::CheckPerformance },
//                 ],
//                 handoff: Default::default(),
//             }
//         }
//     }
// }

// impl eframe::App for NotepadApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         if self.ctx.state == State::Init {
//             self.ctx.state = State::Running;
//         }

//         if self.ctx.state == State::Running {
//             let _ = self.thread.step(&mut self.ctx);
//         }

//         egui::CentralPanel::default().show(ctx, |ui| {
//             if let Some(display) = self.ctx.display_io.as_mut() {
//                 ui.heading(&display.title);
//                 ui.separator();

//                 ui.horizontal(|ui| {
//                     // TODO: task
//                     if self.ctx.write_io.is_none() {
//                         self.ctx.write_io = Some("note.txt".to_string())
//                     }

//                     // TODO: task
//                     if let Some(path) = self.ctx.write_io.as_mut() {
//                         ui.label("File:");
//                         ui.text_edit_singleline(path);
//                     }

//                     // TODO: task
//                     if ui.button("Save").clicked() {
//                         if let Some(path) = self.ctx.write_io.as_ref() {
//                             match std::fs::write(path, &display.body) {
//                                 Ok(()) => {
//                                     display.status = format!(
//                                         "status:\nSaved to {}\n(chars: {})",
//                                         path,
//                                         display.body.chars().count()
//                                     );
//                                 }
//                                 Err(err) => {
//                                     display.status = format!("status:\nSave failed: {}", err);
//                                 }
//                             }
//                         }
//                     }

//                     // TODO: task
//                     if ui.button("Clear").clicked() {
//                         display.body.clear();
//                         display.status = "status:\nCleared".to_string();
//                     }

//                     // TODO: task
//                     if ui.button("Open").clicked() {
//                         if let Some(path) = self.ctx.write_io.as_ref() {
//                             match std::fs::read_to_string(path) {
//                                 Ok(contents) => {
//                                     display.body = contents;
//                                     display.status = format!(
//                                         "status:\nOpened {}\n(chars: {})",
//                                         path,
//                                         display.body.chars().count()
//                                     );
//                                 }
//                                 Err(err) => {
//                                     display.status = format!("status:\nOpen failed: {}", err);
//                                 }
//                             }
//                         }
//                     }        

//                 });

//                 ui.separator();

//                 let response = ui.add(
//                     egui::TextEdit::multiline(&mut display.body)
//                         .desired_rows(20)
//                         .desired_width(f32::INFINITY),
//                 ); 

//                 if response.changed() {
//                     display.status = format!(
//                         "status:\n(system uptime: {}\n(chars: {}",
//                         System::uptime(),
//                         display.body.chars().count() 
//                     );
//                 }

//                 ui.separator();
//                 ui.label(&display.status);

//                 // ctx.request_repaint();
//             }
//         });

//     }

// }
// pub struct NotepadApp {
//     rx: std::sync::mpsc::Receiver<DisplayModel>,
//     display: Option<DisplayModel>,
// }

// impl NotepadApp {
//     pub fn new(rx: Receiver<DisplayModel>) -> Self {
//         Self {
//             rx,
//             display: None,
//         }
//     }
// }


// impl eframe::App for NotepadApp {

//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

//         while let Ok(display) = self.rx.try_recv() {
//             self.display = Some(display);
//         }

//         egui::CentralPanel::default().show(ctx, |ui| {

//             if let Some(display) = &mut self.display {

//                 ui.heading(&display.title);
//                 ui.separator();

//                 ui.add(
//                     egui::TextEdit::multiline(&mut display.body)
//                         .desired_rows(20)
//                         .desired_width(f32::INFINITY)
//                 );

//                 ui.separator();
//                 ui.label(&display.status);

//             } else {

//                 ui.label("Waiting for engine...");

//             }

//         });

//         ctx.request_repaint();
//     }
// }

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

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(display) = self.display.as_mut() {
                ui.heading(&display.title);
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Clear").clicked() {
                        let _ = self.input_tx.send(GuiInput::ClearRequested);
                    }
                });

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