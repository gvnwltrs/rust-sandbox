/*******************************************************************************
 * Rust Regulated Cell Architecture (RCA) 
 * Rust HTTP Server
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
#[allow(unused)]
use std::io::{ Error, Read, Write };

// Timing & performance
#[allow(unused)]
use std::time::Instant;

// Modules
use rust_httpserver::rca_e::*;

// Dependencies
#[allow(unused)]
use std::net::TcpListener;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

// NOTE: Default
// fn main() -> Result<(), Error> {

//     /* 0. Init */

//     // 1. Data Context
//     let mut ctx = Data::give_system_init();
//     println!("\nBoot status: {:#?}\n", ctx);

//     // 2. Thread(s) + task loading
//     // NOTE: add tasks to execute in sequence here
//     let mut current_thread = ProgramThread::Main {
//         counter: 0,
//         tasks: [
//             Cell { id: 0, task: TaskType::None },
//         ],
//         handoff: Default::default(),
//     };

//     ctx.state = State::Halt;
//     println!("\nBoot status: {:#?}\n", ctx);

//     ctx.state = State::Running; 
//     println!("\nBoot status: {:#?}\n", ctx);

//     /*  3. Run Engine */

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


// NOTE: Quick curl only response test
// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:7878")?;
//     println!("Listening on http://127.0.0.1:7878");

//     for stream in listener.incoming() {
//         let mut stream = stream?;
//         let mut buffer = [0u8; 2048];

//         let bytes_read = stream.read(&mut buffer)?;
//         let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

//         println!("Raw request:\n{}", raw_request);

//         // Feed raw_request into RCA-E here
//         let body = format!("RCA-E response\n\n{}", raw_request);

//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
//             body.len(),
//             body
//         );

//         stream.write_all(response.as_bytes())?;
//         stream.flush()?;
//     }

//     Ok(())
// }

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 2048];

        let bytes_read = stream.read(&mut buffer)?;
        let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        let response_model = run_rca_event_flow(raw_request)?;

        let response = format!(
            "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            response_model.status_line,
            response_model.body.len(),
            response_model.body
        );

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}