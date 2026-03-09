/*******************************************************************************
 * Rust Regulated Cell Architecture (RCA) 
 * Author: Gavin Walters
 * Created: 2026-03-06
 * 
 * Description: 
 * Linear Sequential Runtime System 
 * 
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

// Timing & performance
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
// use rust_rca::rca_s::*;
#[allow(unused)]
// use rust_rca::rca_a::*;
#[allow(unused)]
use rust_httpserver::rca_e::*;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> Result<(), Error> {

    /* 0. Init */

    // 1. Data Context
    let mut ctx = Data::give_system_init();
    println!("\nBoot status: {:#?}\n", ctx);

    // 2. Thread(s) + task loading
    // NOTE: add tasks to execute in sequence here
    let mut current_thread = ProgramThread::Main {
        counter: 0,
        tasks: [
            Cell { id: 0, task: TaskType::DisplayData },
            Cell { id: 1, task: TaskType::CheckPerformance },
        ],
        handoff: Default::default(),
    };

    ctx.state = State::Halt;
    println!("\nBoot status: {:#?}\n", ctx);

    ctx.state = State::Running; 
    println!("\nBoot status: {:#?}\n", ctx);

    /*  3. Run Engine */

    loop {

        match ctx.state {

            State::Running => {
                current_thread.step(&mut ctx)?;
                if ctx.debug_mode.is_some()  {
                    println!("\nRuntime status: {:#?}\n", ctx);
                }
            }
            
            _ => {
                ctx.state = State::Shutdown;
                break;
            }

        }

    }

    Ok(())
}