use std::io::Error;
use std::io::ErrorKind;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
#[allow(unused)]
use crate::rca::{ Data, DisplayModel, ThresholdResult, TASK_BUFFER };

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
    U16(u16),
    ThresholdResult(ThresholdResult),
    String(String),
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
    NextCell,
    CommitTick,
}

/* Status: MUTABLE */
#[derive(Debug)]
pub enum TaskType {
    None,
    PassData,
    ReadAdc,
    EvaluateThreshold,
    RenderStatus,
}

/* Status: MUTABLE */
impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
        match self {

            // NOTE: Just a dummy to smoke test
            TaskType::None => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::PassData => {
                ( handoff, Ok(TaskOutput::NextCell) )
            }

            TaskType::ReadAdc => match handoff {
                CellData::U16(sample) => (CellData::U16(sample), Ok(TaskOutput::NextCell)),
                other => (
                    other,
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "ReadAdc: expected CellData::U16",
                    )),
                ),
            },

            TaskType::EvaluateThreshold => match handoff {
                CellData::U16(sample) => {
                    let result = ThresholdResult {
                        sample,
                        above: sample > 700,
                    };

                    (CellData::ThresholdResult(result), Ok(TaskOutput::NextCell))
                }
                other => (
                    other,
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "EvaluateThreshold: expected CellData::U16",
                    )),
                ),
            },

            TaskType::RenderStatus => match handoff {
                CellData::ThresholdResult(result) => {
                    let gpio = if result.above { "HIGH" } else { "LOW " };
                    let uart = if result.above {
                        "ALERT threshold exceeded"
                    } else {
                        "NORMAL adc below threshold"
                    };

                    let text = format!(
                        "ADC={} | GPIO={} | UART=\"{}\"",
                        result.sample, gpio, uart
                    );

                    (CellData::String(text), Ok(TaskOutput::CommitTick))
                }
                other => (
                    other,
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "RenderStatus: expected CellData::ThresholdResult",
                    )),
                ),
            },

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