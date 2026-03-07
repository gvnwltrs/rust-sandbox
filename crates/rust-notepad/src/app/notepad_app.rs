
use crate::rca_s::{ Data, State, ProgramThread, TaskType, Cell };
use sysinfo::System;
use eframe::*;

pub struct NotepadApp {
    pub ctx: Data,
    pub thread: ProgramThread,
}

impl NotepadApp {
    pub fn new() -> Self {
        Self {
            ctx: Data::give_system_init(),
            thread: ProgramThread::Main { 
                counter: 0,
                tasks: [ 
                    Cell { id: 0, task: TaskType::DisplayData },
                    Cell { id: 1, task: TaskType::CheckPerformance },
                ],
                handoff: Default::default(),
            }
        }
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.ctx.state == State::Init {
            self.ctx.state = State::Running;
        }

        if self.ctx.state == State::Running {
            let _ = self.thread.step(&mut self.ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(display) = self.ctx.display_io.as_mut() {
                ui.heading(&display.title);
                ui.separator();

                ui.horizontal(|ui| {
                    if self.ctx.write_io.is_none() {
                        self.ctx.write_io = Some("note.txt".to_string())
                    }

                    if let Some(path) = self.ctx.write_io.as_mut() {
                        ui.label("File:");
                        ui.text_edit_singleline(path);
                    }

                    if ui.button("Save").clicked() {
                        if let Some(path) = self.ctx.write_io.as_ref() {
                            match std::fs::write(path, &display.body) {
                                Ok(()) => {
                                    display.status = format!(
                                        "status:\nSaved to {}\n(chars: {})",
                                        path,
                                        display.body.chars().count()
                                    );
                                }
                                Err(err) => {
                                    display.status = format!("status:\nSave failed: {}", err);
                                }
                            }
                        }
                    }

                    if ui.button("Clear").clicked() {
                        display.body.clear();
                        display.status = "status:\nCleared".to_string();
                    }

                    if ui.button("Open").clicked() {
                        if let Some(path) = self.ctx.write_io.as_ref() {
                            match std::fs::read_to_string(path) {
                                Ok(contents) => {
                                    display.body = contents;
                                    display.status = format!(
                                        "status:\nOpened {}\n(chars: {})",
                                        path,
                                        display.body.chars().count()
                                    );
                                }
                                Err(err) => {
                                    display.status = format!("status:\nOpen failed: {}", err);
                                }
                            }
                        }
                    }        

                });

                ui.separator();

                let response = ui.add(
                    egui::TextEdit::multiline(&mut display.body)
                        .desired_rows(20)
                        .desired_width(f32::INFINITY),
                ); 

                if response.changed() {
                    display.status = format!(
                        "status:\n(system uptime: {}\n(chars: {}",
                        System::uptime(),
                        display.body.chars().count() 
                    );
                }

                ui.separator();
                ui.label(&display.status);

                // ctx.request_repaint();
            }
        });

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