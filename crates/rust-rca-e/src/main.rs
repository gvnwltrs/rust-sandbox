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
use std::io::{ Error, Read, Write };

// Timing & performance
#[allow(unused)]
use std::time::Instant;

// Modules
use rust_rca_e::rca_e::*;

// Dependencies
#[allow(unused)]
use std::net::TcpListener;

/*******************************************************************************
 * Runtime Engine 
******************************************************************************/

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