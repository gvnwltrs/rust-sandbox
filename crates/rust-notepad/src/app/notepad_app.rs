
use crate::rca_s::{ Data, ProgramThread };
use crate::display::{ DisplayModel, GuiInput };
use eframe::*;

use std::sync::mpsc::{Receiver, Sender};
use std::thread;

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