/*******************************************************************************
 * Rust Regulated Cell Architecture (RCA) 
 * Author: Gavin Walters
 * Created: 2026-03-06
 * 
 * Description: 
 * RCA-S Simulation Engine Experiment
 * 
 * 
 * Workflow:
 * Data -> Constraints -> Cells -> Threads -> Engine
 * 
******************************************************************************/

// Data
#[allow(unused)]
use std::marker::PhantomData;
#[allow(unused)]
use std::io::Error;
#[allow(unused)]
use std::time::Instant;

// Modules
#[allow(unused)]
use rust_simulationengine::rca_s::*;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

// fn main() -> Result<(), Error> {

//     let mut ctx = Data::give_system_init();
//     println!("\nBoot status: {:#?}\n", ctx);

//     let mut current_thread = ProgramThread::build_tasks(
//         None,
//         Some([ 
//             Cell { id: 0, task: TaskType::None },
//         ]),
//         None,
//     );

//     ctx.state = State::Halt;
//     println!("\nBoot status: {:#?}\n", ctx);

//     ctx.state = State::Running; 
//     println!("\nBoot status: {:#?}\n", ctx);

//     loop {

//         match ctx.state {

//             State::Running => {
//                 current_thread.step(&mut ctx)?;
//                 if ctx.debug_mode.is_some()  {
//                     println!("\nRuntime status: {:#?}\n", ctx);
//                 }
//             }
            
//             _ => {
//                 ctx.state = State::Shutdown;
//                 break;
//             }

//         }

//     }

//     Ok(())
// }

fn main() -> std::io::Result<()> {
    let mut engine = Engine::give_default();
    engine.access_run(5)?;
    Ok(())
}