
use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
#[allow(unused)]
use crate::rca_e::{ Data, DisplayModel, RequestModel, ResponseModel };

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
    RawRequest(String),
    Request(RequestModel),
    Response(ResponseModel),
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
}

/* Status: MUTABLE */
#[derive(Debug)]
pub enum TaskType {
    None,
    AcceptConnection,
    ReadRequest,
    ParseRequest,
    BuildResponse,
    WriteResponse,
}

/* Status: MUTABLE */
impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, _handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
        match self {

            // NOTE: Just a dummy to smoke test
            TaskType::None => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::AcceptConnection => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::ReadRequest => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::ParseRequest => {
                match _handoff {
                    CellData::RawRequest(raw) => {
                        let mut lines = raw.lines();

                        let request_line = lines.next().unwrap_or_default();
                        let mut parts = request_line.split_whitespace();

                        let method = parts.next().unwrap_or_default().to_string();
                        let path = parts.next().unwrap_or_default().to_string();

                        let mut host = String::new();
                        for line in lines {
                            if let Some(value) = line.strip_prefix("Host: ") {
                                host = value.trim().to_string();
                                break;
                            }
                        }

                        let request = RequestModel {
                            method,
                            path,
                            host,
                            raw,
                        };

                        (CellData::Request(request), Ok(TaskOutput::NextCell))
                    }

                    _ => (
                        CellData::None,
                        Err(Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "expected raw request",
                        )),
                    ),
                }
            }

            // TaskType::BuildResponse => {
            //     match _handoff {
            //         CellData::Request(req) => {
            //             let body = format!(
            //                 "Method: {}\nPath: {}\nHost: {}\n",
            //                 req.method, req.path, req.host
            //             );

            //             let response = ResponseModel {
            //                 status_line: "HTTP/1.1 200 OK".to_string(),
            //                 body,
            //             };

            //             (CellData::Response(response), Ok(TaskOutput::NextCell))
            //         }

            //         _ => (
            //             CellData::None,
            //             Err(Error::new(
            //                 std::io::ErrorKind::InvalidInput,
            //                 "expected parsed request",
            //             )),
            //         ),
            //     }
            // }
            TaskType::BuildResponse => {
                match _handoff {
                    CellData::Request(req) => {
                        let response = if req.method == "GET" && req.path == "/" {
                            let body = format!(
                                "Method: {}\nPath: {}\nHost: {}\n",
                                req.method, req.path, req.host
                            );

                            ResponseModel {
                                status_line: "HTTP/1.1 200 OK".to_string(),
                                body,
                            }
                        } else {
                            let body = format!(
                                "Not Found\n\nMethod: {}\nPath: {}\nHost: {}\n",
                                req.method, req.path, req.host
                            );

                            ResponseModel {
                                status_line: "HTTP/1.1 404 Not Found".to_string(),
                                body,
                            }
                        };

                        (CellData::Response(response), Ok(TaskOutput::NextCell))
                    }

                    _ => (
                        CellData::None,
                        Err(Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "expected parsed request",
                        )),
                    ),
                }
            }

            TaskType::WriteResponse => {
                ( CellData::None , Ok(TaskOutput::None) )
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

    #[test]
    fn test_accept_connection() {
        // mock connection request
        let request = RequestModel {
            method: String::from("GET"),
            path: String::from("/"),
            host: String::from("localhost:7878"),
            raw: String::from("GET / HTTP/1.1\r\nHost: localhost:7878\r\n\r\n"),
        };
        let mut context = Data::give_system_init();
        let _handoff = CellData::Request(request);
        let connection = TaskType::access_task(&TaskType::AcceptConnection, &mut context, _handoff);
        assert!(connection.1.is_ok());
    }
} 