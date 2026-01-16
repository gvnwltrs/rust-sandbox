// "Procedural" macro
// TBD
// Procedural macros are a bit more complicated to set up. However, the
// benefit is then that all processing is done directly with normal Rust code. These macros
// permit use of any syntactic information in unstructured format to generate more code
// structures before compilation.


/* Functional control flow */
// This is the primitive or basic building block to ALL code. 
// Before there were functions and data structures, there was only program flow. 
// These very control flow structures have only evolved over time. 

// Core questions to ask on a project or task:
// 1. What data will the program access and store?
// 2. What input will the program expect?
// 3. What ouput should the program produce?

// Core outlining method for code solution: "stubs" or "stub out"
// 1. create a new project folder 
// 2. create a `Cargo.toml`
// 3. create a `src/main.rs` file
// 4. create the stubs in the `src/main.rs` to outline the target solution

// Writing stubs
// 1. List all program requirements 
// 2. List dependencies or prerequisites for each requirement
// 3. Create a dependency graph from the requirements and dependency lists. For e.g.: 
//      a. store the data types and states
//      b. store the inputs 
//      c. store the other inputs 
//      d. parse the input and store data as some type or collection
//      e. loop while there are remaining requests 
//      f. update the data stores or collections 
//      g. if X happens, now do Y
//      h. based on Y, now adjust for the next request 
//      i. print or log results or statuses 
// 4. Write stubs that implement the depenendency graph 
//
// **NOTE: Using these steps exactly is a reliable method to break down complex problems
// into small problems.**
// 
// Once we setup a 1st draft outline, we just have to fill in the blanks to implement.
// 
// Example stubbed outline (elevatore application):
use std::env;
use std::time::{
    Instant,
    // SystemTime
};
use std::{thread, time};
use std::fs::File;
use std::os::fd::AsFd;
use std::io::{
    self,
    Read,
    Write,
    // prelude::*
};
// use std::process;
use std::cmp;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

extern crate floating_duration;
    use floating_duration::{
        TimeAsFloat, 
        // TimeFormat
    };
extern crate termion;
    use termion::{
        clear, 
        cursor, 
        // style, 
        raw, 
        raw::IntoRawMode, 
        input::TermRead,
        event::Key,
        event::Event,
    };


// NOTE: just a stub, might not use...
pub fn process_floor_requests(floor_requests: &Vec<u64>) {
    if floor_requests.len() == 0 { return; }

    // 5.1 Update location, velocity, and acceleration

    // 5.2 If next floor request in queue is satisfied, then remove from queue

    // 5.3 Adjust motor control to process next floor request 

    // 5.4 Print realtime statistics 

    // tail recursion
    process_floor_requests(&floor_requests);
}

pub fn variable_summary_stats(data: Vec<f64>) -> (f64, f64)
{
   //calculate statistics
   let n = data.len();
   let sum: f64 = data.iter().sum();
   let avg = sum / (n as f64);
   let dev = (
       data.clone().into_iter()
       .map(|v| (v - avg).powi(2))
       .fold(0.0, |a, b| a+b)
       / (n as f64)
   ).sqrt();
   (avg, dev)
}

pub fn variable_summary_print<W: Write + AsFd>(
    stdout: &mut raw::RawTerminal<W>,
    vname: &str,
    avg: f64,
    dev: f64)
{
   //print formatted output
   let _ = write!(stdout, "Average of {:25}{:.6}\r\n", vname, avg);
   let _ = write!(stdout, "Standard deviation of {:14}{:.6}\r\n", vname, dev);
   let _ = write!(stdout, "\r\n");
}

pub fn variable_summary<W: Write + AsFd>(
    stdout: &mut raw::RawTerminal<W>, 
    vname: &str, 
    data: Vec<f64>) 
{
    let (avg, dev) = variable_summary_stats(data);
    variable_summary_print(stdout, vname, avg, dev);
}


pub fn run_simulation() -> (f64, f64, f64, f64, f64){
    // 1. Store, location, velocity, and acceleration state
    let mut location: f64 = 0.0; // meters
    let mut velocity: f64 = 0.0; // meters per second
    let mut acceleration: f64 = 0.0; // meters per second squared

    // 2. Store motor input voltage
    let mut up_input_voltage: f64 = 0.0;
    let mut down_input_voltage: f64 = 0.0;

    // 3. Store input building description and floor requests
    let mut floor_count: u64 = 0;
    let mut floor_height: f64 = 0.0; // meters
    let mut floor_requests: Vec<u64> = Vec::new(); 

    // 4. Parse input and store as building description and floor requests
    let fake_floor_request = 3;
    floor_requests.push(fake_floor_request);
    let buffer = match env::args().nth(1) {
        // If arg with "-" then use the string after as standard input 
        // to read in instead (effectively a no-file manual CLI read in).
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        // If no arg, then use default file name
        None => {
            let fp = "test1.txt";
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        // If arg without "-", treat as the file name we need to read in.
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        }
    };

    // Parse the string's input. Store each line as either floor count,
    // floor height, or floor request, in that order (only 3 lines in the file). 
    for (li, l) in buffer.lines().enumerate() {
        if li == 0 {
            floor_count = l.parse::<u64>().unwrap();
        } else if li == 1 {
            floor_height = l.parse::<f64>().unwrap();
        } else {
            floor_requests.push(l.parse::<u64>().unwrap())
        }
    }

    // 5. Loop while there are remaining floor requests
    // Setup signal handler for Ctrl+C BEFORE entering raw mode
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Attempt to set handler, but don't fail if already set
    // let _ = ctrlc::set_handler(move || {
    //     r.store(false, Ordering::SeqCst);
    // });

    match ctrlc::set_handler(move || 
    {
        r.store(false, Ordering::SeqCst);
    }) {
        Ok(_) => println!("Ctrl+C handler set"),
        Err(e) => eprintln!("Warning: Could not set Ctrl+C handler: {}", e),
    }

    // After entering raw mode:
    let stdin = io::stdin();
    let mut events = stdin.events();

    // Setting up to update location, velocity, and acceleration
    let mut prev_loop_time = Instant::now();
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w-2).unwrap_or(80) as u64;
    let termheight = termsize.map(|(_,h)| h-2).unwrap_or(24) as u64;
    let mut _stdout = io::stdout(); //lock once, instead of once per write
    let mut stdout = match _stdout.lock().into_raw_mode() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Warning: Not running in a terminal. Skipping simulation visualization.");
            eprintln!("Error: {:?}", e);
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }
    };

    // Print startup message
    let _ = write!(stdout, "{}{}Elevator Simulation Starting... (Press Ctrl+C to exit early)\r\n",
                   clear::All, cursor::Goto(1, 1));
    let _ = write!(stdout, "Press any key to start...\r\n");
    stdout.flush().ok();
    thread::sleep(time::Duration::from_millis(1500));

    let mut record_location = Vec::new();
    let mut record_velocity = Vec::new();
    let mut record_acceleration = Vec::new();
    let mut record_voltage = Vec::new();

    while floor_requests.len() > 0 && running.load(Ordering::SeqCst) {
        // Check for Ctrl+C presses
        if let Some(Ok(evt)) = events.next() {
            if let Event::Key(Key::Ctrl('c')) = evt {
                running.store(false, Ordering::SeqCst);
                break;
            }
        }

        // 5.1 Update location, velocity, and acceleration
        // Calculate elapsed time, then update previous time
        let now = Instant::now();
        let dt = now.duration_since(prev_loop_time)
                    .as_fractional_secs();
        prev_loop_time = now;

        record_location.push(location);
        record_velocity.push(velocity);
        record_acceleration.push(acceleration);
        record_voltage.push(up_input_voltage-down_input_voltage);

        location = location + velocity * dt;
        velocity = velocity + acceleration * dt;
        // Analysis of elevator weight found to be 1,200 kg
        acceleration = {
            let _f = (up_input_voltage - down_input_voltage) * 8.0;
            let m = 1200000.0;
            -9.8 + _f/m
        };

        // 5.2 If next floor request in queue is satisfied, then remove from queue
        let next_floor = floor_requests[0];
        if (location - (next_floor as f64)*floor_height).abs() < 0.01 &&
            velocity.abs() < 0.01
        {
            velocity = 0.0;
            floor_requests.remove(0);
        }

        // 5.3 Adjust motor control to process next floor request
        //it will take t seconds to decelerate from velocity v at -1 m/s^2
        let t = velocity.abs() / 1.0;

        //during which time, the carriage will travel d=t * v/2 meters
        //at an average velocity of v/2 before stopping
        let d = t * (velocity/2.0);

        //l = distance to next floor
        let l = (location - (next_floor as f64)*floor_height).abs();

        let target_acceleration = {
            //are we going up?
            let going_up = location < (next_floor as f64)*floor_height;

            //Do not exceed maximum velocity
            if velocity.abs() >= 5.0 {
                //if we are going up and actually going up
                //or we are going down and actually going down
                if (going_up && velocity>0.0)
                || (!going_up && velocity<0.0) {
                0.0
                //decelerate if going in wrong direction
                } else if going_up {
                1.0
                } else {
                -1.0
                }

            //if within comfortable deceleration range and moving in right direction, decelerate
            } else if l < d && going_up==(velocity>0.0) {
                if going_up {
                -1.0
                } else {
                1.0
                }

            //else if not at peak velocity, accelerate
            } else {
                if going_up {
                1.0
                } else {
                -1.0
                }
            }
        };

        let gravity_adjusted_acceleration = target_acceleration + 9.8;
        let target_force = gravity_adjusted_acceleration * 1200000.0;
        let target_voltage = target_force / 8.0;
        if target_voltage > 0.0 {
            up_input_voltage = target_voltage;
            down_input_voltage = 0.0;
        } else {
            up_input_voltage = 0.0;
            down_input_voltage = target_voltage.abs();
        };

        // 5.4 Print realtime statistics
        let _ = write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
        stdout.flush().ok();
        let carriage_floor = (location / floor_height).floor();
        let carriage_floor = if carriage_floor < 1.0 { 0 } else { carriage_floor as u64 };
        let carriage_floor = cmp::min(carriage_floor, floor_count-1);
        let mut terminal_buffer = vec![' ' as u8; (termwidth*termheight) as usize];
        for ty in 0..floor_count
        {
            terminal_buffer[ (ty*termwidth + 0) as usize ] = '[' as u8;
            terminal_buffer[ (ty*termwidth + 1) as usize ] =
                if   (ty as u64)==((floor_count-1)-carriage_floor) { 'X' as u8 }
                else { ' ' as u8 };
            terminal_buffer[ (ty*termwidth + 2) as usize ] = ']' as u8;
            terminal_buffer[ (ty*termwidth + termwidth-2) as usize ] = '\r' as u8;
            terminal_buffer[ (ty*termwidth + termwidth-1) as usize ] = '\n' as u8;
        }
        let stats = vec![
            format!("Carriage at floor {}", carriage_floor+1),
            format!("Location          {:.06}", location),
            format!("Velocity          {:.06}", velocity),
            format!("Acceleration      {:.06}", acceleration),
            format!("Voltage [up-down] {:.06}", up_input_voltage-down_input_voltage),
        ];
        for sy in 0..stats.len()
        {
            for (sx,sc) in stats[sy].chars().enumerate()
            {
                terminal_buffer[ sy*(termwidth as usize) + 6 + sx ] = sc as u8;
            }
        }
        let _ = write!(stdout, "{}", String::from_utf8(terminal_buffer).unwrap());
        stdout.flush().unwrap();

        thread::sleep(time::Duration::from_millis(10));
    }

    // 6. Print summary or return
    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();

    // Check if interrupted by Ctrl+C
    if !running.load(Ordering::SeqCst) {
        let _ = write!(stdout, "\r\nSimulation interrupted by user (Ctrl+C). Printing summary...\r\n\r\n");
    }
    variable_summary(&mut stdout, "location", record_location);
    variable_summary(&mut stdout, "velocity", record_velocity);
    variable_summary(&mut stdout, "acceleration", record_acceleration);
    variable_summary(&mut stdout, "voltage", record_voltage);
    stdout.flush().unwrap();

    // Make borrow-checker happy (unused for now) by sinking into "let _"
    // Or just return the tuple:
    (
        location, 
        velocity, 
        acceleration, 
        up_input_voltage, 
        down_input_voltage
    )
}