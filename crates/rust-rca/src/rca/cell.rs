
use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
use crate::rca::{ 
    DataPlane, 
};

/* Status: MUTABLE */
#[allow(unused)]
pub const CELLS: usize = 2;

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
    Byte(u8), 
    // Add cell data types here
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
    pub task: Task,
}

/* Status: FREEZE */
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}

/* Status: FREEZE */
impl Cell {
    pub fn default() -> [Self; CELLS] {
        let tasks: [Self; CELLS] = core::array::from_fn(|i| Cell {
            id: i,
            task: Task::Default,
        });
        tasks
    }

    pub fn execute(&mut self, context: &DataPlane, handoff: CellData) -> Result<CellData, Error> {
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
pub enum Task {
    Default,
    DoubleValue,
    // Add tasks here
}

/* Status: MUTABLE */
impl Task {
    pub fn access_task(&self, _ctx: &DataPlane, _handoff: CellData) ->  Result<CellData, Error> {
        match self {

            Task::Default => {
                let transform = CellData::Byte(0x2A); // 42
                Ok(transform)
            }

            Task::DoubleValue => {
                match _handoff {
                    CellData::Byte(x) => {
                        let result = x + x;
                        let transform = CellData::Byte(result); // 84
                        Ok(transform)
                    }

                    _ => {
                        Ok(_handoff)
                    }
                }
            }

            // Add task procedures here

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