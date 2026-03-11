use std::io::Error;
use sysinfo::System;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_a::{ Data, DisplayModel, TASK_BUFFER };

/*******************************************************************************
 * (1) Cell Data 
******************************************************************************/

/* Cells 
 * Description: Each cell can get access to the system context or data, but it cannot modify the context. Only the engine has authority to modify state. 
 * Nature: Each cell HAS-A task
 */

/* Status: MUTABLE */
#[derive(Debug, PartialEq, Clone)]
pub enum CellData {
    None,
    String(String),
    U8(u8),
    U32(u32),
    I32(i32),
    F32(f32),
    F64(f64),
    DisplayData(DisplayModel),
}

impl Default for CellData {
    fn default() -> Self {
        CellData::None
    }
}

/* Status: FREEZE */
#[derive(Debug)]
pub struct Cell {
    pub id: usize,
    pub task: TaskType,
}

/* Status: FREEZE */
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}

/* Status: FREEZE */
impl Cell {
    pub fn default() -> [Self; TASK_BUFFER] {
        let tasks: [Self; TASK_BUFFER] = core::array::from_fn(|i| Cell {
            id: i,
            task: TaskType::None,
        });
        tasks
    }

    pub fn execute(&mut self, context: &mut Data, handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
       self.task.access_task(context, handoff) 
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;

    #[test]
    fn smoke_test() {
        assert!(true);
    }
} 

/*******************************************************************************
 * (2) Tasks 
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