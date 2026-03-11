#[allow(unused)]
use std::io::{Error, ErrorKind};
use std::time::Instant;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_s::{
    Data,
    ProgramThread,
    TaskType,
    Cell,
    CellData,
    State,
    SystemData,
    DisplayModel,
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
            ctx: Data::default(),
            sys: SystemData::default(),
        }
    }

    pub fn access_run(&mut self, max_ticks: u32) -> Result<(), Error> {
        self.ctx.state = State::Running;

        while self.ctx.state == State::Running && self.sys.sim.tick < max_ticks {
            let start = Instant::now();

            self.prepare_cycle();
            self.run_thread()?;
            self.regulate_results()?;

            let elapsed = start.elapsed();
            self.ctx.perf = Some(format!("Tick {} runtime: {:?}", self.sys.sim.tick, elapsed));

            let rendered = self.render_cycle_output()?;
            println!("{rendered}\n");
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

        self.sys.sim.tick += 1;
    }

    fn run_thread(&mut self) -> Result<(), Error> {
        let mut thread = ProgramThread::build_tasks(
            Some(0),
            Some([
                Cell { id: 0, task: TaskType::ApplyVelocity },
                Cell { id: 1, task: TaskType::ClampBounds },
                Cell { id: 2, task: TaskType::RenderSnapshot },
            ]),
            Some(CellData::SimState(self.sys.sim.clone())),
        );

        self.sys.rendered = None;

        while !thread.is_finished() {
            thread.step(&mut self.ctx)?;

            let tag = thread.access_handoff().access_tag();
            println!("DEBUG handoff after step: {tag}");

            match thread.access_handoff() {
                CellData::SimState(sim) => {
                    self.sys.sim = sim.clone();
                }
                CellData::String(rendered) => {
                    self.sys.rendered = Some(rendered.clone());
                }
                CellData::None => {}
                _ => {}
            }
        }

        let final_handoff = thread.take_handoff();
        println!("DEBUG final handoff: {}", final_handoff.access_tag());

        match final_handoff {
            CellData::String(rendered) => {
                self.sys.rendered = Some(rendered.clone());

                self.ctx.write_io = Some(rendered.clone());
                self.ctx.display_io = Some(DisplayModel {
                    title: "RCA-S Simulation Engine".into(),
                    body: rendered,
                    status: "OK".into(),
                });
            }
            CellData::SimState(sim) => {
                self.sys.sim = sim;
            }
            CellData::None => {}
            _ => {}
        }

        Ok(())
    }

    fn regulate_results(&mut self) -> Result<(), Error> {
        if self.ctx.display_io.is_none() {
            if let Some(rendered) = self.sys.rendered.clone() {
                self.ctx.write_io = Some(rendered.clone());
                self.ctx.display_io = Some(DisplayModel {
                    title: "RCA-S Simulation Engine".into(),
                    body: rendered,
                    status: "OK".into(),
                });
            }
        }

        if self.ctx.display_io.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Engine: display_io was not populated",
            ));
        }

        self.ctx.state = State::Running;
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
