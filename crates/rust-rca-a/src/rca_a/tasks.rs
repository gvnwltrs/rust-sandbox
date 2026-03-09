
use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_a::{ Data, CellData, DisplayModel };

/*******************************************************************************
 * 4. Tasks 
******************************************************************************/

/* Tasks 
 * Description: Tasks help formulate cells. 
 * Nature: Each task HAS-A type and operation/behavior.
 */

/* Status: MUTABLE */ 
#[derive(Debug)]
pub enum TaskOutput {
    None,
    MutateReadIO,
    MutateWriteIO,
    MutateDisplayIO,
    MutatePerf,
    MutateLogs,
    NextCell,
}

/* Status: MUTABLE */
#[derive(Debug)]
pub enum TaskType {
    None,
    DisplayData,
    CheckPerformance,
}

/* Status: MUTABLE */
impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
        match self {

            // NOTE: Just a dummy to smoke test
            TaskType::None => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::DisplayData => {
                let data = DisplayModel { 
                        title: format!("Test"),
                        body: String::new(),
                        status: format!("status: \n(system_uptime: {}), ", System::uptime()) 
                };
                ( CellData::DisplayData(data), Ok(TaskOutput::MutateDisplayIO) ) 
            }

            TaskType::CheckPerformance => {
                let uptime = System::uptime();
                ( CellData::String(format!("uptime: {}, TBD...", uptime)), Ok(TaskOutput::MutatePerf) )
            }

        }
    }
}