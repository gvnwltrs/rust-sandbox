use std::io::Error;

#[allow(unused)]
use crate::rca::{ Data, SystemData, ProgramThread, TaskType, Cell, CellData, State, Mode };

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
    fn try_run_engine(&mut self) -> Result<(), Error>;
}

impl PrimaryRunner for Engine {
     fn give_default() -> Self {
        Self {
            ctx: Data::default(),
            sys: SystemData::default(),
        }
    }

    fn try_run_engine(&mut self) -> Result<(), Error> {
        println!("\nStage Change: {:#?}\n", self.ctx);

        let mut current_thread = ProgramThread::build_tasks(
            None,
            Some([ 
                Cell { id: 0, task: TaskType::None },
            ]),
            None,
        );

        self.ctx.state = State::Halt;
        println!("\nState Change: {:#?}\n", self.ctx);

        self.ctx.state = State::Running; 
        println!("\nState Change: {:#?}\n", self.ctx);

        loop {

            match self.ctx.state {
                State::Running => {
                    current_thread.step(&mut self.ctx)?;

                    if let Mode::Debug =  self.ctx.mode {
                        println!("\nRuntime status: {:#?}\n", self.ctx);
                    }

                    if current_thread.is_finished() {
                        self.ctx.state = State::Shutdown;
                        return Ok(());
                    }

                }

                _ => { self.ctx.state = State::Shutdown; return Ok(()); }

            }

        }

    }
}

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/
