use std::io::Error;
use std::io::ErrorKind;

/* Project Dependencies */
use crate::rca_s::{ Data, DisplayModel, TASK_BUFFER, SimulationState };

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
    SimState(SimulationState),
}

impl Default for CellData {
    fn default() -> Self {
        CellData::None
    }
}

impl CellData {
     pub fn access_tag(&self) -> &'static str {
        match self {
            CellData::None => "None",
            CellData::String(_) => "String",
            CellData::SimState(_) => "SimState",
            _ => "Unknown",
        }
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
    MutateReadIO,
    MutateWriteIO,
    MutateDisplayIO,
    MutatePerf,
    MutateLogs,
    NextCell,
    CommitTick,
}

/* Status: MUTABLE */
#[derive(Debug)]
pub enum TaskType {
    None,
    ApplyVelocity,
    ClampBounds,
    RenderSnapshot,
}

/* Status: MUTABLE */
impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, _handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
        match self {

            // NOTE: Just a dummy to smoke test
            TaskType::None => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::ApplyVelocity => {
                match _handoff {
                    CellData::SimState(mut sim) => {
                        for entity in &mut sim.entities {
                            entity.x += entity.vx;
                            entity.y += entity.vy;
                        }

                        (CellData::SimState(sim), Ok(TaskOutput::NextCell))
                    }

                    other => (
                        other,
                        Err(Error::new(
                            ErrorKind::InvalidData,
                            "ApplyVelocity: expected CellData::SimState",
                        )),
                    ),
                }
            }

            TaskType::ClampBounds => {
                match _handoff {
                    CellData::SimState(mut sim) => {
                        for entity in &mut sim.entities {
                            if entity.x < 0.0 {
                                entity.x = 0.0;
                                entity.vx = -entity.vx;
                            }
                            if entity.x > sim.width {
                                entity.x = sim.width;
                                entity.vx = -entity.vx;
                            }

                            if entity.y < 0.0 {
                                entity.y = 0.0;
                                entity.vy = -entity.vy;
                            }
                            if entity.y > sim.height {
                                entity.y = sim.height;
                                entity.vy = -entity.vy;
                            }
                        }

                        (CellData::SimState(sim), Ok(TaskOutput::NextCell))
                    }

                    other => (
                        other,
                        Err(Error::new(
                            ErrorKind::InvalidData,
                            "ClampBounds: expected CellData::SimState",
                        )),
                    ),
                }
            }

            TaskType::RenderSnapshot => {
                match _handoff {
                    CellData::SimState(sim) => {
                        let mut body = format!("Tick {}\n", sim.tick);

                        for entity in &sim.entities {
                            body.push_str(&format!(
                                "Entity {}: (x={:.1}, y={:.1}, vx={:.1}, vy={:.1})\n",
                                entity.id, entity.x, entity.y, entity.vx, entity.vy
                            ));
                        }

                        (
                            CellData::String(body.trim_end().to_string()),
                            Ok(TaskOutput::CommitTick),
                        )
                    }

                    other => (
                        other,
                        Err(Error::new(
                            ErrorKind::InvalidData,
                            "RenderSnapshot: expected CellData::SimState",
                        )),
                    ),
                }
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