
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
            } else {
                ui.heading("rust-notepad");
                ui.label("No display yet.");
            }

            ctx.request_repaint();
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