/* Micro-kernel space (Loop Engine privelage only):
* Apply returned outputs to ctx.
* This is the missing link that makes "returns" actually do something.
*/
use std::io::Error;

#[allow(unused)]
use crate::rca::{ 
    DataPlane, 
    ConfigData,
    ReadData,
    WriteData,
    PerfData,
    LogData,
    CellInfo,
    ActivityInfo,
    DisplayInfo,
    SystemData, 
    ControlPlane,
    State, 
    Mode,
    Event,
    CELLS,
    Cell, 
    CellData, 
    Task, 
    ProgramThread, 
    Effect,
};

/*******************************************************************************
 * (1) Default 
******************************************************************************/

#[derive(Debug)]
pub struct Engine {
    pub ctx: DataPlane,
    pub ctl: ControlPlane,
    pub sys: SystemData,
}

pub trait PrimaryRunner {
    fn give_default() -> Self;
    fn access_initializing_status(&self);
    fn access_running_status(&self, efx: &CellData);
    fn access_shutdown_status(&self);
    fn try_run_engine(&mut self) -> Result<(), Error>;
}

impl PrimaryRunner for Engine {
     fn give_default() -> Self {
        Self {
            // ctx: DataPlane::default(),
            ctx: DataPlane {
                config: ConfigData::None,
                read_io: ReadData::None,
                write_io: WriteData::None,
                perf: PerfData::None,
                logs: LogData::None,
                cells: CellInfo { count: CELLS },
                activity: ActivityInfo::default(),
                display: DisplayInfo::default(),
            },
            ctl: ControlPlane::default(),
            sys: SystemData::default(),
        }
    }


    fn access_initializing_status(&self) {
        println!("\n>>>\nControl: {:#?}\n\nData: {:#?}\n", self.ctl, self.ctx);
    }

    fn access_running_status(&self, efx: &CellData) {
        println!("\n>>>\nControl: {:#?}\n\nEffect: {:#?}\n\nData: {:#?}\n<<<\n", self.ctl, efx, self.ctx);
    }

    // TODO: Modify to be more relevant to shutdown details. 
    fn access_shutdown_status(&self) {
        println!("\n>>>\nControl: {:#?}\n\nData: {:#?}\n", self.ctl, self.ctx);
    }

    fn try_run_engine(&mut self) -> Result<(), Error> {
        self.access_initializing_status();

        let mut current_thread = ProgramThread::build_tasks(
            None,
            Some([ 
                Cell { id: 0, task: Task::Default },
                Cell { id: 1, task: Task::DoubleValue },
            ]),
            None,
        );

        self.ctl.state = State::Halt;
        self.access_initializing_status();

        self.ctl.state = State::Idle;
        self.access_initializing_status();

        self.ctl.state = State::Running; 
        self.access_initializing_status();

        loop {

            match self.ctl.state {
                State::Running => {
                    let effect = current_thread.step(&self.ctx)?;
                    self.ctx.activity = effect.activity;

                    if let Mode::Debug =  self.ctl.mode {
                        self.access_running_status(effect.handoff);
                    }

                    if effect.finished {
                        self.ctl.state = State::Shutdown;
                        self.ctx.activity = ActivityInfo::default();
                        self.access_shutdown_status();
                        return Ok(());
                    }

                }

                _ => { 
                    self.ctl.state = State::Shutdown; 
                    self.access_shutdown_status();
                    return Ok(()); 
                }

            }

        }

    }
}

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/

/* Example:
trait MyAppRunner {
    fn foo();
    fn bar();
}

impl MyAppRunner for Engine {
    ...
}
*/
