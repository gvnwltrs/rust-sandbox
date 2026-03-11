use std::io::Error;
use std::io::ErrorKind;

#[allow(unused)]
use crate::rca::{ 
    Data, 
    SystemData, 
    DisplayModel,
    ProgramThread, 
    TaskType, 
    Cell, 
    CellData, 
    State 
};

/*******************************************************************************
 * (1) Default 
******************************************************************************/

#[derive(Debug)]
pub struct Engine {
    pub ctx: Data,
    pub sys: SystemData,
}

pub trait PrimaryRunner {
    fn give_default() -> Self;
    fn try_run_engine(&mut self, max_ticks: u32) -> Result<(), Error>;
    fn prepare_cycle(&mut self); 
    fn run_thread(&mut self) -> Result<(), Error>;
    fn regulate_results(&mut self) -> Result<(), Error>;
    fn render_cycle_output(&self) -> Result<String, Error>;
}

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/

impl PrimaryRunner for Engine {
    fn give_default() -> Self {
        Self {
            ctx: Data::default(),
            sys: SystemData::default(),
        }
    }

    fn try_run_engine(&mut self, max_ticks: u32) -> Result<(), Error> {
        self.ctx.state = State::Running;

        while self.ctx.state == State::Running && self.sys.hw.timer.tick < max_ticks {
            self.prepare_cycle();
            self.run_thread()?;
            self.regulate_results()?;

            let output = self.render_cycle_output()?;
            println!("{output}");
        }

        self.ctx.state = State::Shutdown;
        Ok(())
    }

    fn prepare_cycle(&mut self) {
        self.ctx.write_io = None;
        self.ctx.display_io = None;
        self.ctx.task_desc = None;
        self.ctx.cur_cell_id = Some(0);
        self.ctx.prev_cell_id = Some(0);

        self.sys.hw.timer.tick += 1;
        self.sys.rendered = None;
        self.sys.last_result = None;

        self.sys.hw.adc.value = match self.sys.hw.timer.tick {
            1 => 200,
            2 => 450,
            3 => 800,
            4 => 300,
            5 => 900,
            _ => 0,
        };
    }

    fn run_thread(&mut self) -> Result<(), Error> {
        let mut thread = ProgramThread::build_tasks(
            Some(0),
            Some([
                Cell { id: 0, task: TaskType::ReadAdc },
                Cell { id: 1, task: TaskType::EvaluateThreshold },
                Cell { id: 2, task: TaskType::RenderStatus },
            ]),
            Some(CellData::U16(self.sys.hw.adc.value)),
        );

        while !thread.is_finished() {
            thread.step(&mut self.ctx)?;
        }

        let final_handoff = thread.take_handoff();

        match final_handoff {
            CellData::String(rendered) => {
                self.sys.rendered = Some(rendered.clone());
                self.ctx.write_io = Some(rendered.clone());
                self.ctx.display_io = Some(DisplayModel {
                    title: "RCA-S Embedded Mock".into(),
                    body: rendered,
                    status: "OK".into(),
                });
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Engine: final handoff was not render output",
                ));
            }
        }

        Ok(())
    }

    fn regulate_results(&mut self) -> Result<(), Error> {
        let sample = self.sys.hw.adc.value;
        let above = sample > self.sys.hw.adc.threshold;

        self.sys.hw.gpio.pin_high = above;
        self.sys.hw.uart.last_tx = Some(if above {
            "ALERT threshold exceeded".to_string()
        } else {
            "NORMAL adc below threshold".to_string()
        });

        self.sys.last_result = Some(crate::rca::ThresholdResult {
            sample,
            above,
        });

        if self.ctx.display_io.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Engine: display_io was not populated",
            ));
        }

        Ok(())
    }

    fn render_cycle_output(&self) -> Result<String, Error> {
        let Some(display) = self.ctx.display_io.as_ref() else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Engine: missing display output",
            ));
        };

        Ok(display.body.clone())
    }
}