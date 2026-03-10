
use crate::rca_e::{ Data, ResponseModel, ProgramThread, TaskType, Cell, CellData, State };

/*******************************************************************************
 * (1) Default 
******************************************************************************/

/*******************************************************************************
 * (2) Add custom engine here  
******************************************************************************/

pub fn run_rca_event_flow(raw_request: String) -> std::io::Result<ResponseModel> {
    let mut ctx = Data::give_system_init();

    let mut current_thread = ProgramThread::build_tasks(
            None,
            Some([ 
                Cell { id: 0, task: TaskType::ParseRequest }, 
                Cell { id: 1, task: TaskType::BuildResponse }, 
            ]),
            Some(CellData::RawRequest(raw_request)),
    );

    ctx.state = State::Running;

    while ctx.state == State::Running {
        current_thread.step(&mut ctx)?;

        if current_thread.is_finished() {
            ctx.state = State::Shutdown;
        }
    }

    match current_thread.take_handoff() {
        CellData::Response(response) => Ok(response),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "expected response from RCA-E flow",
        )),
    }

}