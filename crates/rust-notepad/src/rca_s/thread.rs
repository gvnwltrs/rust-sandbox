
use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
#[allow(unused)]
use eframe::egui;
#[allow(unused)]
use crate::display::DisplayModel;
use crate::rca_s::{ Data, State, TaskOutput, Cell, CellData };

/*******************************************************************************
 * 3. Threads 
******************************************************************************/

/* Status: MUTABLE */
#[allow(unused)]
pub const THREADS: usize = 1;

/* Status: MUTABLE */
#[allow(unused)]
pub const TASK_BUFFER: usize = 2;

/* Status: MUTABLE */
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
    GUI,
}

/* Status: FREEZE Main / MUTABLE (additional threads) */
impl ProgramThread {

    /* Desc: (1) call function execute, (2) update state */
    pub fn step(&mut self, ctx: &mut Data) -> Result<(), Error> { 
        match self {

            ProgramThread::Main { counter, tasks , handoff } => {

                // (1) Execute task
                if *counter >= TASK_BUFFER {
                    ctx.cur_cell_id = None;
                    ctx.prev_cell_id = None;
                    ctx.task_desc = None;
                    ctx.state = State::Shutdown;
                    return Ok(());
                }

                ctx.cur_cell_id = Some(*counter); 
                ctx.task_desc = Some(format!("{:#?}", tasks[*counter].task));

                // Literally handoff the data here and replaces current value with default for the old owner.
                let handoff_transfer: CellData = std::mem::take(handoff);

                // Move the handoff to the new owner.
                let out: (CellData, Result<TaskOutput, Error>) = tasks[*counter].execute(ctx, handoff_transfer);

                // Back to owning cell data. Update the handoff with the results from out.
                let _handoff = out.0;
                let task_output = out.1?;


                // (2) Modify state (optional)
                let handover = ctx.mutate_state((_handoff, task_output))?;

                *handoff = match handover { 
                    Some(celldata) => celldata,
                    None => CellData::None
                };
                *counter += 1;

                if ctx.cur_cell_id > Some(1) {
                    ctx.prev_cell_id = Some(*counter - 1);
                }

                return Ok(());
            }

            ProgramThread::GUI => { return Ok(()); }

        }
    }

}
