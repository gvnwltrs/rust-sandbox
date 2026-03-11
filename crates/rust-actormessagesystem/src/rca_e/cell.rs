use std::io::Error;
use std::io::ErrorKind;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_e::{ Data, Message, ActorId, MessageKind, DisplayModel, TASK_BUFFER };

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
    Message(Message),
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
            CellData::Message(_) => "Message",
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
    NextCell,
    CommitEvent,
}

/* Status: MUTABLE */
#[derive(Debug)]
pub enum TaskType {
    None,
    PassData,
    LoadNextMessage,
    DispatchActor,
    RenderEvent,
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

            TaskType::LoadNextMessage => match handoff {
                CellData::Message(msg) => (CellData::Message(msg), Ok(TaskOutput::NextCell)),
                other => (
                    other,
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "LoadNextMessage: expected CellData::Message",
                    )),
                ),
            },

            TaskType::DispatchActor => match handoff {
                CellData::Message(msg) => {
                    let text = match (msg.to, msg.kind) {
                        (ActorId::A, MessageKind::Start) => {
                            "Actor A handled Start\nActor A sent Ping to Actor B".to_string()
                        }
                        (ActorId::B, MessageKind::Ping) => {
                            "Actor B handled Ping\nActor B sent Ping to Actor C".to_string()
                        }
                        (ActorId::C, MessageKind::Ping) => {
                            "Actor C handled Ping\nActor C sent Done to Actor A".to_string()
                        }
                        (ActorId::A, MessageKind::Done) => {
                            "Actor A handled Done\nSystem complete".to_string()
                        }
                        _ => {
                            return (
                                CellData::None,
                                Err(Error::new(
                                    ErrorKind::InvalidData,
                                    "DispatchActor: unsupported message route",
                                )),
                            )
                        }
                    };

                    (CellData::String(text), Ok(TaskOutput::NextCell))
                }
                other => (
                    other,
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "DispatchActor: expected CellData::Message",
                    )),
                ),
            },

            TaskType::RenderEvent => match handoff {
                CellData::String(text) => (CellData::String(text), Ok(TaskOutput::CommitEvent)),
                other => (
                    other,
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "RenderEvent: expected CellData::String",
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