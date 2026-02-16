
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

fn main() -> Result<(), Error>{

    let system = give_init();
    println!("System status: {:#?}", system);

    /* Engine */
    loop {
        break;
    }

    Ok(())
}

/*******************************************************************************
 *                      Linear Sequential Runtime System 
******************************************************************************/

/*******************************************************************************
 * 1. Data
******************************************************************************/

const THREADS: usize = 1;
const TASKS: usize = 1;

#[allow(unused)]
const EXECUTION_THRESHOLD: f64 = 1.;

#[derive(Debug, PartialEq)]
#[allow(unused)]
enum Stub {
    TBD,
    WIP,
}

#[derive(Debug, PartialEq)]
#[allow(unused)]
struct Data {
    config: Option<String>,
    sys_perf: f64,
    threads: [Stub; THREADS],
    prev_task: Option<Stub>,
    task: Option<Stub>,
    payload: Option<Stub>,
}

/*******************************************************************************
 * 2. States 
******************************************************************************/

#[derive(Debug, PartialEq)]
#[allow(unused)]
enum State {
    Initialized,
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
enum Thread {
    Main {
        counter: usize,
        tasks: [Stub; TASKS],
    },
}

/*******************************************************************************
 * 4. Tasks 
******************************************************************************/

#[allow(unused)]
fn give_task_1() -> Result<(), Error>{
    Ok(())
}

/*******************************************************************************
 * 5. Engine 
******************************************************************************/

#[allow(unused)]
fn give_init() -> Result<(), Error>{
    Ok(())
}