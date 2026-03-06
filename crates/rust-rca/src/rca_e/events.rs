use std::io::Error;

#[allow(unused)]
use sysinfo::System;

#[allow(unused)]
use std::time::SystemTime;

#[allow(unused)]
use std::fmt::write;

/* Project Dependencies */
// use eframe::egui;

/*******************************************************************************
 * 1. Data
 * 
 * Establish data endpoints. 
 * Establish & confirm complete data. 
 *
******************************************************************************/

/* Status: MUTABLE */
#[allow(unused)]
pub const THREADS: usize = 1;

/* Status: MUTABLE */
#[allow(unused)]
pub const TASK_BUFFER: usize = 2;

/* Status: MUTABLE */
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

/* Status: 
    - fields: FREEZE 
    - values: MUTABLE
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
    pub cur_cell_id: Option<usize>,         // Introspection into current activity
    pub prev_cell_id: Option<usize>,            // Access index: Current cell can access previous cell generated data
    pub debug_mode: Option<String>,
    pub task_desc: Option<String>,
    pub state: State,                   // System state
}

/* Status: FREEZE */
impl Data {
    pub fn give_system_init() -> Self {
        Self {
            read_io: None,
            write_io: None,
            display_io: None,
            config: None,
            perf: None,
            logs: None,
            cur_cell_id: Some(0),
            prev_cell_id: Some(0),
            debug_mode: Some(String::from("Default")),
            task_desc: None,
            state: State::Init,
        }
    }

    /* Micro-kernel space (Loop Engine privelage only):
    * Apply returned outputs to ctx.
    * This is the missing link that makes "returns" actually do something.
    */

    /* Status: MUTABLE */
    pub fn mutate_state(&mut self, _in: TaskOutput) -> Result<(), Error> {
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

/* Status: FREEZE */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum State {
    Init,       // (0)
    Halt,       // (1)
    Running,    // (2) 
    Failure,    // (3)
    Degraded,   // (4)
    Shutdown,   // (5)
}

/*******************************************************************************
 * 3. Threads 
******************************************************************************/

/* Status: FREEZE */
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum ProgramThread {
    Main {
        counter: usize,
        tasks: [Cell; TASK_BUFFER],
        handoff: CellData, 
    },
}

/* Status: FREEZE */
impl ProgramThread {
    pub fn step(&mut self, ctx: &mut Data) -> Result<(), Error> { 
        match self {

            ProgramThread::Main { counter, tasks , handoff } => {

                if *counter >= TASK_BUFFER {
                    ctx.cur_cell_id = None;
                    ctx.prev_cell_id = None;
                    ctx.task_desc = None;
                    ctx.state = State::Shutdown;
                    return Ok(());
                }

                ctx.cur_cell_id = Some(*counter); 
                ctx.task_desc = Some(format!("{:#?}", tasks[*counter].task));

                // Literally handoff the data here and replaces current value with default for the old owner.
                let handoff_transfer: CellData = std::mem::take(handoff);

                // Move the handoff to the new owner.
                let out: (CellData, Result<TaskOutput, Error>) = tasks[*counter].execute(ctx, handoff_transfer);

                // Back to owning cell data. Update the handoff with the results from out.
                *handoff = out.0;

                ctx.mutate_state(out.1?)?;
                *counter += 1;

                if ctx.cur_cell_id > Some(1) {
                    ctx.prev_cell_id = Some(*counter - 1);
                }

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


/* Status: 
    - enum: FREEZE
    - value: MUTABLE
*/
#[derive(Debug)]
pub enum TaskOutput {
    None,
    MutateReadIO(),
    MutateWriteIO(),
    MutateDisplayIO(String),
    MutatePerf(String),
    MutateLogs(String),
    MutateState(State),
}

/* Status: MUTABLE */
#[derive(Debug)]
pub enum TaskType {
    None,
    DisplayData,
    CheckPerformance,
}

/* Status: MUTABLE */
impl TaskType {
    pub fn access_task(&self, _ctx: &mut Data, handoff: CellData) -> (CellData, Result<TaskOutput, Error>) {
        match self {

            // NOTE: Just a dummy to smoke test
            TaskType::None => {
                ( CellData::None , Ok(TaskOutput::None) )
            }

            TaskType::DisplayData => {
                ( CellData::None, Ok(TaskOutput::MutateDisplayIO(format!("{:#?}", handoff))) )
            }

            TaskType::CheckPerformance => {
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

/* Status: FREEZE */
#[derive(Debug, PartialEq, Clone)]
pub enum CellData {
    None,
    String(String),
    U8(u8),
    U32(u32),
    I32(i32),
    F32(f32),
    F64(f64),
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