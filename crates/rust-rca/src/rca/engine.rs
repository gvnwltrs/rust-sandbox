/* Micro-kernel space (Loop Engine privelage only):
* Apply returned outputs to ctx.
* This is the missing link that makes "returns" actually do something.
*/
use std::io::Error;

#[allow(unused)]
use crate::rca::{ 
    DataPlane, 
    SystemData, 
    ControlPlane,
    State, 
    Mode,
    Event,
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
    fn access_status(&self);
    fn try_run_engine(&mut self) -> Result<(), Error>;
}

impl PrimaryRunner for Engine {
     fn give_default() -> Self {
        Self {
            ctx: DataPlane::default(),
            ctl: ControlPlane::default(),
            sys: SystemData::default(),
        }
    }

    fn access_status(&self) {
        println!("\nData: {:#?} | Control: {:#?}\n", self.ctx, self.ctl);
    }

    fn try_run_engine(&mut self) -> Result<(), Error> {
        self.access_status();

        let mut current_thread = ProgramThread::build_tasks(
            None,
            Some([ 
                Cell { id: 0, task: Task::Default },
            ]),
            None,
        );

        self.ctl.state = State::Halt;
        self.access_status();

        self.ctl.state = State::Idle;
        self.access_status();

        self.ctl.state = State::Running; 
        self.access_status();

        loop {

            match self.ctl.state {
                State::Running => {
                    let effect = current_thread.step(&self.ctx)?;
                    self.ctx.activity = effect.activity;

                    if let Mode::Debug =  self.ctl.mode {
                        self.access_status();
                    }

                    if effect.finished {
                        self.ctl.state = State::Shutdown;
                        self.access_status();
                        return Ok(());
                    }

                }

                _ => { 
                    self.ctl.state = State::Shutdown; 
                    self.access_status();
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