use std::io::Error;

#[allow(unused)]
use crate::rca_e::{ Data, SystemData, ProgramThread, TaskType, Cell, CellData, State };

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
        println!("\nBoot status: {:#?}\n", self.ctx);

        let mut current_thread = ProgramThread::build_tasks(
            None,
            Some([ 
                Cell { id: 0, task: TaskType::None },
            ]),
            None,
        );

        self.ctx.state = State::Halt;
        println!("\nBoot status: {:#?}\n", self.ctx);

        self.ctx.state = State::Running; 
        println!("\nBoot status: {:#?}\n", self.ctx);

        loop {

            match self.ctx.state {

                State::Running => {
                    current_thread.step(&mut self.ctx)?;
                    if self.ctx.debug_mode.is_some()  {
                        println!("\nRuntime status: {:#?}\n", self.ctx);
                    }
                }
                
                _ => {
                    self.ctx.state = State::Shutdown;
                    break;
                }

            }

        }

        Ok(())
    }
}

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/

