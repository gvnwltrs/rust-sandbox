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

// Timing
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
use rust_sysutil::*;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

fn main() -> Result<(), Error>{

    /* Init */

    // Context
    let mut ctx = give_init()?;
    println!("\nSystem status: {:#?}\n", ctx);

    // Thread(s) + task loading
    // NOTE: add tasks to execute in sequence here
    let mut current_thread = ProgramThread::Main {
        counter: 0,
        tasks: [
            TaskFn { id: 0, input: TaskInput::None, func: take_task_input },
            TaskFn { id: 1, input: TaskInput::MutateData, func: take_task_input },
            TaskFn { id: 2, input: TaskInput::ReportState, func: take_task_input },
        ],
    };

    ctx.state = State::Idle;
    println!("\nSystem status: {:#?}\n", ctx);
    ctx.state = State::Running; 
    println!("\nSystem status: {:#?}\n", ctx);

    loop {

        match &mut current_thread {

            ProgramThread::Main { counter, tasks } => { 
                if *counter >= TASK_BUFFER {
                    ctx.state = State::Shutdown;
                    println!("\nReport: {:#?}\n", ctx);
                    break;
                }

                match ctx.state {
                    
                    State::Running => {
                        let out = tasks[*counter].execute(&mut ctx)?;
                        let _ = mutate_state(&mut ctx, out);
                        *counter += 1;

                        if ctx.state == State::Report {
                            println!("\nCounter: {:#?}\n", *counter);
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
        }

    }
    Ok(())
}