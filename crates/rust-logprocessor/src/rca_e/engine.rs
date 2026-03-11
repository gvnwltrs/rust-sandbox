use std::io::Error;
use std::io::ErrorKind;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_e::{
    Data,
    ProgramThread,
    TaskType,
    Cell,
    CellData,
    State,
    SystemData,
    SummaryState,
    DisplayModel,
    ClassifiedLog,
    LogLevel,
    TASK_BUFFER,
};

/*******************************************************************************
 * (1) Default 
******************************************************************************/

#[derive(Debug)]
pub struct Engine {
    pub ctx: Data,
    pub sys: SystemData,
}

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/

impl Engine {
    pub fn give_default() -> Self {
        Self {
            ctx: Data::give_system_init(),
            sys: SystemData::default(),
        }
    }

    pub fn access_line(&mut self, line: &str) -> Result<String, Error> {
        self.prepare_cycle(line);
        self.run_thread()?;
        self.regulate_results()?;
        self.render_cycle_output()
    }

    fn prepare_cycle(&mut self, line: &str) {
        self.ctx.state = State::Running;
        self.ctx.read_io = Some(line.to_string());
        self.ctx.write_io = None;
        self.ctx.display_io = None;
        self.ctx.task_desc = None;
        self.ctx.cur_cell_id = Some(0);
        self.ctx.prev_cell_id = Some(0);

        self.sys.raw_line = Some(crate::rca_e::RawLine {
            text: line.to_string(),
        });
        self.sys.record = None;
        self.sys.classified = None;
    }

    // fn run_thread(&mut self) -> Result<(), Error> {
    //     let mut thread = ProgramThread::build_tasks(
    //         Some(0),
    //         Some([
    //             Cell { id: 0, task: TaskType::ParseLogLine },
    //             Cell { id: 1, task: TaskType::ClassifyLogLine },
    //             Cell { id: 2, task: TaskType::RenderDisplay },
    //         ]),
    //         Some(CellData::None),
    //     );

    //     while !thread.is_finished() {
    //         thread.step(&mut self.ctx)?;
    //     }

    //     let final_handoff = thread.take_handoff();

    //     // match final_handoff {
    //     //     CellData::Classified(classified) => {
    //     //         self.sys.classified = Some(classified);
    //     //     }
    //     //     CellData::Record(record) => {
    //     //         self.sys.record = Some(record);
    //     //     }
    //     //     CellData::None => {}
    //     //     _ => {}
    //     // }

    //     match final_handoff {
    //         CellData::String(rendered) => {
    //             self.ctx.write_io = Some(rendered.clone());
    //             self.ctx.display_io = Some(DisplayModel {
    //                 title: "RCA-E Log Processor".into(),
    //                 body: rendered,
    //                 status: "OK".into(),
    //             });
    //         }
    //         CellData::Classified(classified) => {
    //             self.sys.classified = Some(classified);
    //         }
    //         _ => {}
    //     }

    //     Ok(())
    // }

    fn run_thread(&mut self) -> Result<(), Error> {
        let mut thread = ProgramThread::build_tasks(
            Some(0),
            Some([
                Cell { id: 0, task: TaskType::ParseLogLine },
                Cell { id: 1, task: TaskType::ClassifyLogLine },
                Cell { id: 2, task: TaskType::RenderDisplay },
            ]),
            Some(CellData::None),
        );

        while !thread.is_finished() {
            thread.step(&mut self.ctx)?;

            match thread.access_handoff() {
                CellData::Record(record) => {
                    self.sys.record = Some(record.clone());
                }
                CellData::Classified(classified) => {
                    self.sys.classified = Some(classified.clone());
                }
                _ => {}
            }
        }

        let final_handoff = thread.take_handoff();

        match final_handoff {
            CellData::String(rendered) => {
                self.ctx.write_io = Some(rendered.clone());
                self.ctx.display_io = Some(DisplayModel {
                    title: "RCA-E Log Processor".into(),
                    body: rendered,
                    status: match self.sys.classified.as_ref() {
                        Some(c) if c.is_alert => "Alert".into(),
                        _ => "OK".into(),
                    },
                });
            }
            CellData::Classified(classified) => {
                self.sys.classified = Some(classified);
            }
            CellData::Record(record) => {
                self.sys.record = Some(record);
            }
            CellData::None => {}
            _ => {}
        }

        Ok(())
    }

    fn regulate_results(&mut self) -> Result<(), Error> {
        let Some(classified) = self.sys.classified.clone() else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Engine: missing final classified result",
            ));
        };

        self.update_summary(&classified);
        self.update_apex_io(&classified);
        self.ctx.state = State::Idle;

        Ok(())
    }

    fn update_summary(&mut self, classified: &ClassifiedLog) {
        self.sys.summary.total += 1;

        match classified.level {
            LogLevel::Info => {
                self.sys.summary.info_count += 1;
            }
            LogLevel::Warn => {
                self.sys.summary.warn_count += 1;
            }
            LogLevel::Error => {
                self.sys.summary.error_count += 1;
                self.sys.summary.last_error = Some(classified.message.clone());
            }
            LogLevel::Unknown => {
                self.sys.summary.unknown_count += 1;
            }
        }
    }

    fn update_apex_io(&mut self, classified: &ClassifiedLog) {
        let level_str = match classified.level {
            LogLevel::Info => "INFO ",
            LogLevel::Warn => "WARN ",
            LogLevel::Error => "ERROR",
            LogLevel::Unknown => "UNKWN",
        };

        let alert_line = if classified.is_alert {
            "Alert raised\n"
        } else {
            ""
        };

        let body = format!(
            "[{}] {}\n{}Summary: total={} info={} warn={} error={} unknown={}{}",
            level_str,
            classified.message,
            alert_line,
            self.sys.summary.total,
            self.sys.summary.info_count,
            self.sys.summary.warn_count,
            self.sys.summary.error_count,
            self.sys.summary.unknown_count,
            match &self.sys.summary.last_error {
                Some(msg) => format!(" last_error=\"{}\"", msg),
                None => String::new(),
            }
        );

        self.ctx.display_io = Some(DisplayModel {
            title: "RCA-E Log Processor".to_string(),
            body: body.clone(),
            status: if classified.is_alert {
                "Alert".to_string()
            } else {
                "OK".to_string()
            },
        });

        self.ctx.write_io = Some(body);
    }

    fn render_cycle_output(&self) -> Result<String, Error> {
        let Some(display) = self.ctx.display_io.as_ref() else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Engine: display_io was not populated",
            ));
        };

        Ok(display.body.clone())
    }
}