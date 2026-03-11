use std::io::Error;

/* Project Dependencies */
use crate::rca::{ Data, Cell, CellData };

/*******************************************************************************
 * (1) Threads 
******************************************************************************/

/* Status: MUTABLE */
#[allow(unused)]
pub const THREADS: usize = 1;

/* Status: MUTABLE */
#[allow(unused)]
pub const TASK_BUFFER: usize = 1;

/* Status: rcaLE */
#[allow(unused)]
pub const EXECUTION_THRESHOLD: f64 = 1.;  // Units in ms

/* Status: FREEZE Main / MUTABLE (additional threads) */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum ProgramThread {
    Main {
        counter: usize,
        tasks: [Cell; TASK_BUFFER],
        handoff: CellData, 
    },
}

/* Status: FREEZE Main / MUTABLE (additional threads) */
impl ProgramThread {

    pub fn build_tasks(ctr: Option<usize>, tsks: Option<[Cell; TASK_BUFFER]>, ho: Option<CellData>) -> Self {
        ProgramThread::Main {
            counter: if let Some(count) = ctr { count } else { 0 },
            tasks: if let Some(inner_tasks) = tsks {
                inner_tasks 
            } else {
                Cell::default()
            },
            handoff: if let Some(inner_data) = ho {
                inner_data    
            } else {
                Default::default()
            },
        }
        
    }

    /* Desc: (1) call function execute, (2) update state */
    pub fn step(&mut self, ctx: &mut Data) -> Result<(), Error> { 
        if self.is_finished() { 
            ctx.task_desc = Default::default();
            return Ok(());
        }

        match self {

            ProgramThread::Main { counter, tasks , handoff } => {

                ctx.task_desc = format!("{:#?}", tasks[*counter].task);

                // Literally handoff the data here and replaces current value with default for the old owner.
                let handoff_transfer: CellData = std::mem::take(handoff);

                // Move the handoff to the new owner then back to owning cell data. Update the handoff with the results from out.
                *handoff = tasks[*counter].execute(ctx, handoff_transfer)?;
                *counter += 1;

                Ok(())
            }

        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            ProgramThread::Main { counter, tasks, .. } => *counter >= tasks.len(),
        }
    }

    pub fn take_handoff(&mut self) -> CellData {
        match self {
            ProgramThread::Main { handoff, .. } => std::mem::take(handoff),
        }
    }

    pub fn access_handoff(&self) -> &CellData {
        match self {
            ProgramThread::Main { handoff, .. } => handoff,
        }
    }

}