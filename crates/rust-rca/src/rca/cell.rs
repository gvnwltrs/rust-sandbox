
use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
#[allow(unused)]
use crate::rca::{ Data, DisplayData, TASK_BUFFER };

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

    pub fn execute(&mut self, context: &mut Data, handoff: CellData) -> Result<CellData, Error> {
       self.task.access_task(context, handoff) 
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
pub enum TaskType {
    None,
    PassData,
}

/* Status: MUTABLE */
impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, handoff: CellData) ->  Result<CellData, Error> {
        match self {

            TaskType::None => {
                Ok(handoff)
            }

            TaskType::PassData => {
                Ok(handoff)
            }

        }

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