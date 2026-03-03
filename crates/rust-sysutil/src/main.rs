/*******************************************************************************
 * Rust CLI System Utils 
 * Author: Gavin Walters
 * Created: 2026-02-27
 * 
 * Description: 
 * Linear Sequential Runtime System 
 * 
 * Provides CLI tools for Linux Operating System
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
use rust_sysutil::*;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> Result<(), Error> {

    /* 0. Init */

    // 1. Data Context
    let mut ctx = Data::give_system_init();
    println!("\nSystem status: {:#?}\n", ctx);

    // 2. Thread(s) + task loading
    // NOTE: add tasks to execute in sequence here
    let mut current_thread = ProgramThread::Main {
        counter: 0,
        tasks: [
            Cell { id: 0, task: TaskType::AccessReport },
            Cell { id: 1, task: TaskType::CreateMsg },
            Cell { id: 2, task: TaskType::AccessReport },
            Cell { id: 3, task: TaskType::DisplayMsg },
            Cell { id: 4, task: TaskType::AccessReport },
        ],
        handoff: Default::default(),
    };

    ctx.state = State::Idle;
    println!("\nSystem status: {:#?}\n", ctx);

    ctx.state = State::Running; 
    println!("\nSystem status: {:#?}\n", ctx);

    /*  3. Run Engine */

    loop {

        match ctx.state {

            State::Running => {
                current_thread.step(&mut ctx)?;
                if ctx.state == State::Report {
                    println!("\nReport: {:#?}\n", ctx);
                    ctx.state = State::Running;
                }
            }
            
            _ => {
                ctx.state = State::Shutdown;
                println!("\nReport: {:#?}\n", ctx);
                break;
            }

        }

    }

    Ok(())
}