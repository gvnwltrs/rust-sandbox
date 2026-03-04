use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

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
pub const TASK_BUFFER: usize = 6;

#[allow(unused)]
pub const EXECUTION_THRESHOLD: f64 = 1.;  // Units in ms

/* Apex Data:
 * The primary goal of this data is to represent the state of the overall system. 
 * The data here is only the essential data for which to affect the system with,
 * or to provide the status & performance of the system. "Affecting the system" 
 * means to produce data such that it produces a particular desired result in line
 * with the purporse of the system. This could be graphical display, system device interaction,
 * pure logging or data etc. By constraining the overall apex data to just these,
 * we can essentially affect any part of the system while maintaining clarity for 
 * what we are trying to do strictly within the constraints or scope of the design. 
 */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct Data {
    pub config: Option<String>,         // (0) Init state: details for initalization & configuration of system behavior
    pub read_io: Option<String>,        // (2) Running state: import data (e.g. file system or API call) 
    pub write_io: Option<String>,       // (2) Running state: export data (e.g. file system or API call)
    pub display_io: Option<String>,     // (2) Running state: utilizing system terminal output or display drivers
    pub perf: Option<String>,           // (2) Running state: system information details 
    pub logs: Option<[String; 100]>,    // (2, 3, 4, 5) Running, Failure, Degraded, Shutdown state: Logs for any event during running state  
    pub prev_cell_id: usize,        // Access index: Current cell can access previous cell generated data
    pub debug_mode: Option<String>,
    pub state: State,                   // System state
}

impl Data {
    pub fn give_system_init() -> Self {
        Self {
            read_io: None,
            write_io: None,
            display_io: None,
            config: None,
            perf: None,
            logs: None,
            prev_cell_id: 0,
            debug_mode: Some(String::from("Default")),
            state: State::Init,
        }
    }

    /* Micro-kernel space (Loop Engine privelage only):
    * Apply returned outputs to ctx.
    * This is the missing link that makes "returns" actually do something.
    */
    pub fn mutate_state(&mut self, _in: TaskOutput, id: usize) -> Result<(), Error> {
        self.prev_cell_id = id;
        match _in {
            TaskOutput::None => { Ok(()) }
            TaskOutput::MutateState(next_state) => { self.state = next_state; Ok(()) }
            TaskOutput::MutateDisplayIO(data) => { self.display_io = Some(data); Ok(()) }
            TaskOutput::MutatePerf(data) => { self.perf = Some(data); Ok(()) }
            _ => Ok(())
        }
    }
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
    Init,       // (0)
    Idle,       // (1)
    Running,    // (2) 
    Failure,    // (3)
    Degraded,   // (4)
    Shutdown,   // (5)
}

/*******************************************************************************
 * 3. Threads 
******************************************************************************/

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum ProgramThread {
    Main {
        counter: usize,
        tasks: [Cell; TASK_BUFFER],
        handoff: CellData, 
    },
}

impl ProgramThread {
    pub fn step(&mut self, ctx: &mut Data) -> Result<(), Error> { 
        match self {

            ProgramThread::Main { counter, tasks , handoff } => {

                if *counter >= TASK_BUFFER {
                    ctx.state = State::Shutdown;
                    println!("\nReport: {:#?}\n", ctx);
                    return Ok(());
                }

                // Literally handoff the data here and place default for the old owner.
                let handoff_transfer = std::mem::take(handoff);
                // Move the handoff to the new owner.
                let out = tasks[*counter].execute(ctx, handoff_transfer);
                // Update the handoff with the results from out.
                *handoff = out.0;

                ctx.mutate_state(out.1?, *counter)?;
                *counter += 1;
                return Ok(());
            }

        }
    }

}

/*******************************************************************************
 * 4. Tasks 
******************************************************************************/

/* Tasks 
 * Description: Tasks help formulate cells. 
 * Nature: Each task HAS-A type and operation/behavior.
 */

#[derive(Debug)]
pub enum TaskOutput {
    None,
    MutateReadIO(),
    MutateWriteIO(),
    MutateDisplayIO(String),
    MutatePerf(String),
    MutateLogs(),
    MutateState(State),
}

#[derive(Debug)]
pub enum TaskType {
    None,
    AccessReport,
    EmitData,
    DisplayData,
    CheckPerfomance,
}

impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
        match self {

            // NOTE: Just a dummy to smoke test
            TaskType::None => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::AccessReport => {
                ( handoff, Ok(TaskOutput::None) )
            }

            TaskType::EmitData => {
                ( CellData::String(format!("My string.")), Ok(TaskOutput::None) )
            }

            TaskType::DisplayData => {
                ( CellData::None, Ok(TaskOutput::MutateDisplayIO(format!("{:#?}", handoff))) )
            }

            TaskType::CheckPerfomance => {
                let uptime = System::uptime();
                ( CellData::None, Ok(TaskOutput::MutatePerf(format!("uptime: {}, TBD...", uptime))) )
            }

        }
    }
}

/*******************************************************************************
 * 5. Cell Data 
******************************************************************************/

/* Cells 
 * Description: Each cell can get access to the system context or data, but it cannot modify the context. Only the engine has authority to modify state. 
 * Nature: Each cell HAS-A task
 */

#[derive(Debug, PartialEq, Clone)]
pub enum CellData {
    None,
    String(String),
}

impl Default for CellData {
    fn default() -> Self {
        CellData::None
    }
}

#[derive(Debug)]
pub struct Cell {
    pub id: usize,
    pub task: TaskType,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}

impl Cell {
    pub fn execute(&mut self, context: &mut Data, handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
       self.task.access_task(context, handoff) 
    }
}