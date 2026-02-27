
/*******************************************************************************
 * Rust Patterns 
 * Author: Gavin Walters
 * Created: 2026-02-16
 * 
 * Description: 
 * 
 * Gives examples of various patterns I like to use:
 * - Linear Sequential Runtime System (LSRS) -- default
 * - Dynamic LSRS (DLSRS)
 * - Event-Driven Runtime System (EDRS) 
 * 
 * Workflow:
 * Data -> States -> Threads -> Tasks -> Engine
 * 
******************************************************************************/

// Data
#[allow(unused)]
use std::marker::PhantomData;

// Errors
use std::io::Error;
// use std::fmt::Error;
// use std::error::Error;
// use core::error::Error;

// Timing
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
use rust_patterns::*;


/*******************************************************************************
 *                      Linear Sequential Runtime System 
******************************************************************************/

/*******************************************************************************
 * 1. Data
 * 
 * Establish data endpoints. 
 * Establish & confirm complete data. 
 *
******************************************************************************/

#[allow(unused)]
const THREADS: usize = 1;

#[allow(unused)]
const BUFFER: usize = 1;

#[allow(unused)]
const EXECUTION_THRESHOLD: f64 = 1.;  // Units in ms

#[derive(Debug, PartialEq)]
#[allow(unused)]
struct Data {
    data_1: u32,
    config: Option<String>,
    sys_perf: f64,
    payload: Option<String>,
    state: State,
}

/*******************************************************************************
 * 2. States 
******************************************************************************/

#[derive(Debug, PartialEq)]
#[allow(unused)]
enum State {
    Init,
    Idle,
    Running,
    Failure,
    Degraded,
    Shutdown,
}

/*******************************************************************************
 * 3. Threads 
******************************************************************************/
#[derive(Debug, PartialEq)]
#[allow(unused)]
enum ProgramThread {
    Main {
        counter: usize,
        tasks: [Task; BUFFER],
    },
}

/*******************************************************************************
 * 4. Tasks 
******************************************************************************/

// Helpers 

#[derive(Debug, PartialEq)]
struct TaskFn {
    id: usize,
    func: fn() -> Result<(), Error>,
}

#[derive(Debug, PartialEq)]
enum Task {
    NoArg(TaskFn),
}

impl Task {
    fn execute(&self) -> Result <(), Error> {
        match self {
            Task::NoArg(f) => (f.func)(),
        }

    }
}

#[allow(unused)]
enum PerfTask {
    StartTime,
    EndTime,
}

#[allow(unused)]
fn give_task_perf_time_elapse(p: PerfTask) -> Result<usize, Error>{
    match p {
        PerfTask::StartTime => Ok(0),
        PerfTask::EndTime => Ok(0),
    }
}

/* NOTE:
 * Real task implmentations exist outside of main in modules, crates, or libraries. 
 */

fn access_mock_task() -> Result<(), Error> {
    println!("\n===================Mock task executing=======================\n");
    Ok(())
}

/*******************************************************************************
 * 5. Engine 
******************************************************************************/

#[allow(unused)]
fn give_init() -> Result<Data, Error>{
    Ok(Data {
        data_1: 0,
        config: None,
        sys_perf: 0.,
        payload: None,
        state: State::Init,
    })
}

fn main() -> Result<(), Error>{

    /* Init */

    // Context
    let mut ctx = give_init()?;
    println!("System status: {:#?}", ctx);

    // Thread(s) + task load
    let mut current_thread = ProgramThread::Main {
        counter: 0,
        tasks: [
            Task::NoArg(TaskFn { id: 0, func: access_mock_task } ),
        ],
    };

    ctx.state = State::Idle;

    /* Running */
    ctx.state = State::Running; 
    loop {
        /* NOTE: Should structure task handler here to avoid duplication of functions. Previously,
         * I had been creating functions in main that call functions in implmentation. 
         */

        println!("Counter: {:#?}", current_thread);
        match &mut current_thread { // FIXME: This is a bit clunky or sloppy. Might be a more
                                 // idiomatic way to accomplish this...

            ProgramThread::Main { counter, tasks } => { 
                match (*counter < BUFFER, &ctx.state) {
                    (true, State::Running) => {
                        tasks[*counter].execute()?;
                        *counter += 1;
                    }
                    _ => {
                        ctx.state = State::Shutdown;
                        println!("Report: {:#?}", ctx);
                        break;
                    }
                }

            }

        }
    }

    Ok(())
}

