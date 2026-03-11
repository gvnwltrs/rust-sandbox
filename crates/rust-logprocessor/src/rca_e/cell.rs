use std::io::Error;
use std::io::ErrorKind;

/* Project Dependencies */
use crate::rca_e::{ TASK_BUFFER, Data, LogLevel, LogRecord, ClassifiedLog };

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
    Record(LogRecord),
    Classified(ClassifiedLog),
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
    UpdateSummary,
    RaiseAlert(String),
}

/* Status: MUTABLE */
#[derive(Debug)]
pub enum TaskType {
    None,
    ParseLogLine,
    ClassifyLogLine,
    RenderDisplay,
}

/* Status: MUTABLE */
impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, _handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
        match self {

            // NOTE: Just a dummy to smoke test
            TaskType::None => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

             TaskType::ParseLogLine => {
                let Some(raw) = _ctx.read_io.as_ref() else {
                    return (
                        CellData::None,
                        Err(Error::new(ErrorKind::InvalidInput, "ParseLogLine: ctx.read_io is None")),
                    );
                };

                let mut parts = raw.splitn(2, ' ');
                let level_str = parts.next().unwrap_or_default();
                let message = parts.next().unwrap_or_default().trim().to_string();

                let level = match level_str {
                    "INFO" => LogLevel::Info,
                    "WARN" => LogLevel::Warn,
                    "ERROR" => LogLevel::Error,
                    _ => LogLevel::Unknown,
                };

                let record = LogRecord {
                    level,
                    message,
                };

                (CellData::Record(record), Ok(TaskOutput::NextCell))
            }

            TaskType::ClassifyLogLine => {
                match _handoff {
                    CellData::Record(record) => {
                        let is_alert = matches!(record.level, LogLevel::Error);

                        let classified = ClassifiedLog {
                            level: record.level.clone(),
                            message: record.message.clone(),
                            is_alert,
                        };

                        if is_alert {
                            (
                                CellData::Classified(classified),
                                Ok(TaskOutput::RaiseAlert(record.message)),
                            )
                        } else {
                            (
                                CellData::Classified(classified),
                                Ok(TaskOutput::UpdateSummary),
                            )
                        }
                    }

                    other => {
                        (
                            other,
                            Err(Error::new(ErrorKind::InvalidData, "ClassifyLogLine: expected CellData::Record")),
                        )
                    }

                }
            }

            TaskType::RenderDisplay => {
                match _handoff {

                    CellData::Classified(classified) => {

                        let level_str = match classified.level {
                            LogLevel::Info => "INFO ",
                            LogLevel::Warn => "WARN ",
                            LogLevel::Error => "ERROR",
                            LogLevel::Unknown => "UNKWN",
                        };

                        let alert_line = if classified.is_alert {
                            "Alert raised\n"
                        } else {
                            ""
                        };

                        let body = format!(
                            "[{}] {}\n{}",
                            level_str,
                            classified.message,
                            alert_line,
                        );

                        (
                            CellData::String(body),
                            Ok(TaskOutput::UpdateSummary)
                        )
                    }

                    other => {
                        (
                            other,
                            Err(Error::new(
                                ErrorKind::InvalidData,
                                "RenderDisplay: expected ClassifiedLog"
                            ))
                        )
                    }
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