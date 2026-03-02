use std::io::Error;
use sysinfo::System;

/*******************************************************************************
 * 1. Data
 * 
 * Establish data endpoints. 
 * Establish & confirm complete data. 
 *
******************************************************************************/

#[allow(unused)]
pub const THREADS: usize = 1;

#[allow(unused)]
pub const TASK_BUFFER: usize = 3;

#[allow(unused)]
pub const EXECUTION_THRESHOLD: f64 = 1.;  // Units in ms

#[derive(Debug, PartialEq)]
pub enum Unit {
    String(String),
    U32(u32),
    U64(u64),
    State,
}

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct Data {
    pub data: Option<Unit>,
    pub read_io: Option<String>,
    pub write_io: Option<String>,
    pub display_io: Option<String>,
    pub config: Option<String>,
    pub perf: Option<f64>,
    pub logs: Option<[String; 100]>,
    pub state: State,
}

// NOTE: Idea for future domain implementatoins:
// - MCU projects:
//  - Data contains GPIO, I2C, SPI, UART, Registers, Memory Addresses, etc.

/*******************************************************************************
 * 2. States 
******************************************************************************/

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum State {
    Init,
    Idle,
    Running,
    Report,
    Failure,
    Degraded,
    Shutdown,
}

/*******************************************************************************
 * 3. Threads 
******************************************************************************/

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum ProgramThread {
    Main {
        counter: usize,
        tasks: [TaskFn; TASK_BUFFER],
    },
}

/*******************************************************************************
 * 4. Tasks 
******************************************************************************/

/* Handlers */ 

#[derive(Debug)]
pub enum TaskInput {
    None,
    MutateData,
    ReportState,
    Text,
    Uptime,
    PerfMS,
}

#[derive(Debug)]
pub enum TaskOutput {
    None,
    MutateData(Unit),
    NextState(State),
    Text(String),
    PerfMs(f64),
}

#[derive(Debug)]
pub struct TaskFn {
    pub id: usize,
    pub input: TaskInput,
    pub func: fn(&mut Data, &TaskInput) -> Result<TaskOutput, Error>,
}

impl PartialEq for TaskFn {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}

impl TaskFn {
    pub fn execute(&self, d: &mut Data) -> Result<TaskOutput, Error> {
       (self.func)(d, &self.input) 
    }
}

/* Functions */

pub fn give_init() -> Result<Data, Error>{
    Ok(Data {
        data: None,
        read_io: None,
        write_io: None,
        display_io: None,
        config: None,
        perf: None,
        logs: None,
        state: State::Init,
    })
}

pub fn take_task_input(_ctx: &mut Data, _in: &TaskInput) -> Result<TaskOutput, Error> {
    match _in {

        TaskInput::None => {
            Ok(TaskOutput::None)
        }

        TaskInput::Text => {
            Ok(TaskOutput::Text(format!("\n===================Mock task executing=======================\n")))
        }

        TaskInput::MutateData => {
            Ok(TaskOutput::MutateData(Unit::String(format!("test data"))))
        }

        TaskInput::ReportState => {
            Ok(TaskOutput::NextState(State::Report))
        }

        TaskInput::Uptime => {
            Ok(TaskOutput::Text(format!("\nUptime: {:#?}\n", System::uptime())))
        }

        _ => {
            Ok(TaskOutput::None)
        }

    }
}

// Apply returned outputs to ctx.
// This is the missing link that makes "returns" actually do something.
pub fn mutate_state(ctx: &mut Data, _in: TaskOutput) -> Result<(), Error> {
    match _in {
        TaskOutput::None => { Ok(()) }
        TaskOutput::NextState(next_state) => { ctx.state = next_state; Ok(()) }
        TaskOutput::MutateData(data) => { ctx.data = Some(data); Ok(()) }
        TaskOutput::Text(s) => {
            // your choice: display_io vs read_io vs logs, etc.
            ctx.display_io = Some(s);
            Ok(())
        }
        TaskOutput::PerfMs(ms) => {
            ctx.perf = Some(ms);
            Ok(())
        }
    }
}