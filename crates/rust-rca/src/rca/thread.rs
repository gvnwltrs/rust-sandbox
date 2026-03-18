use std::io::Error;

/* Project Dependencies */
use crate::rca::{ 
    DataPlane, 
    ActivityInfo,
    Cell, 
    CellData,
    CELLS,
};

/*******************************************************************************
 * (1) Threads 
******************************************************************************/

/* Status: MUTABLE */
#[allow(unused)]
pub const THREADS: usize = 1;

/* Status: MUTABLE */
#[allow(unused)]
pub const EXECUTION_THRESHOLD: f64 = 1.;  // Units in ms

/* Status: MUTABLE */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct Effect<'a> {
    pub activity: ActivityInfo,
    pub handoff: &'a CellData,
    pub finished: bool,
}

/* Status: MUTABLE */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum ProgramThread {
    /* Status: FREEZE */
    Main {
        counter: usize,
        tasks: [Cell; CELLS],
        handoff: CellData, 
    },
}

/* Status: MUTABLE */
impl ProgramThread {

    pub fn build_tasks(ctr: Option<usize>, tsks: Option<[Cell; CELLS]>, ho: Option<CellData>) -> Self {
        /* Status: FREEZE */
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

        // Add threads here
        
    }

    /* Desc: (1) call function execute, (2) update state handled by engine */
    pub fn step(&mut self, ctx: &DataPlane) -> Result<Effect<'_>, Error> { 

        match self {

            ProgramThread::Main { counter, tasks , handoff } => {
                // if *counter >= tasks.len() {
                //     let activity = ActivityInfo {
                //         description: String::new(),
                //     };

                //     return Ok(Effect {
                //         activity,
                //         handoff,
                //         finished: true,
                //     });
                // }

                let activity = ActivityInfo {
                    description: format!("{:#?}", tasks[*counter].task),
                };

                // Literally handoff the data here from the previous cell to giver ownership to the new cell since thread owns the data.
                let handoff_transfer: CellData = std::mem::take(handoff);

                // Move the handoff to the new owner then back to owning cell data. Update the handoff with the results from out.
                *handoff = tasks[*counter].execute(ctx, handoff_transfer)?;
                *counter += 1;

                let finished = self.is_finished(); 

                return Ok(Effect {
                    activity, 
                    handoff: self.access_handoff(finished),
                    finished: finished,
                });
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

    pub fn access_handoff(&self, _finished: bool) -> &CellData {
        // if finished {
        //     match self {
        //         ProgramThread::Main { handoff, .. } => handoff,
        //     }
        // } else {
        //     &CellData::None
        // }
        match self {
            ProgramThread::Main { handoff, .. } => handoff,
        }
    }
}