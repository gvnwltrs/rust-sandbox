use std::io::Error;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_a::{ Data, State, Events, TaskType, TaskOutput, Cell, CellData, DisplayModel };

/*******************************************************************************
 * (1) Threads 
******************************************************************************/

/* Status: MUTABLE */
#[allow(unused)]
pub const THREADS: usize = 1;

/* Status: MUTABLE */
#[allow(unused)]
pub const TASK_BUFFER: usize = 1;

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
}

/* Status: FREEZE Main / MUTABLE (additional threads) */
impl ProgramThread {

    pub fn build_tasks(ctr: Option<usize>, tsks: Option<[Cell; TASK_BUFFER]>, ho: Option<CellData>) -> Self {
        ProgramThread::Main {
            counter: if let Some(x) = ctr { x } else { 0 },
            tasks: if let Some(x) = tsks {
                x
            } else {
                [ 
                    Cell { id: 0, task: TaskType::None }, 
                ]
            },
            handoff: if let Some(x) = ho {
                x    
            } else {
                Default::default()
            },
        }
        
    }

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

                // Thread owns continuation policy
                *handoff = match task_output {
                    TaskOutput::None => CellData::None,
                    TaskOutput::NextCell => _handoff,
                    // ADD HERE
                };

                *counter += 1;

                if ctx.cur_cell_id > Some(1) {
                    ctx.prev_cell_id = Some(*counter - 1);
                }

                return Ok(());
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

}