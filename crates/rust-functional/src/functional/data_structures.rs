/* 
 * Functional data structures 
 **/
use serde::{Serialize,Deserialize};

 // Enums are also known as "tagged unions". This allows us to define data
 // that may have multiple structures or use cases. 
 #[derive(Clone,Serialize,Deserialize,Debug)]
 pub enum MotorInput {
    Up { voltage: f64 },
    Down { voltage: f64 },
 }

 // Note: trait items always share the visibility of their trait
 // , so you don't have to use 'pub' in the impl in this case
 pub trait MotorForce {
    fn calculate_force(&self) -> f64; // contract for impl to implement
 }

 impl MotorForce for MotorInput {
    fn calculate_force(&self) -> f64 {
        match *self {
            MotorInput::Up { voltage: v } => { v * 8.0 }
            MotorInput::Down { voltage: v } => { v * -8.0 }
        }
    }
 }

 pub trait MotorVoltage {
    fn voltage(&self) -> f64;
 }

impl MotorVoltage for MotorInput {
    fn voltage(&self) -> f64 {
        match *self {
            MotorInput::Up { voltage: v } => { v }
            MotorInput::Down { voltage: v } => { -v }
        }
    }
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct ElevatorSpecification {
    pub floor_count: u64,
    pub floor_height: f64,
    pub carriage_weight: f64,
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct ElevatorState {
    pub timestamp: f64,
    pub location: f64,
    pub velocity: f64,
    pub acceleration: f64,
    pub motor_input: MotorInput,
}

pub type FloorRequests = Vec<u64>;

pub trait MotorController {
    fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState);
    fn poll(&mut self, esp: ElevatorState, dst: u64);
}   

pub struct SimpleMotorController {
    pub esp: ElevatorSpecification
}

#[allow(unused)]
impl MotorController for SimpleMotorController {
    fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState) {
        self.esp = esp;
    }
    fn poll(&mut self, esp: ElevatorState, dst: u64) {}
}

pub trait DataRecorder {
    fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState);
    fn poll(&mut self, esp: ElevatorState, dst: u64);
    fn summary(&mut self);
}   

#[derive(Debug)]
#[allow(unused)]
pub struct SimpleDataRecorder {
    pub esp: ElevatorSpecification,
    pub record_location: Vec<f64>,
    pub record_velocity: Vec<f64>,
    pub record_acceleration: Vec<f64>,
    pub record_voltage: Vec<f64>,
}

#[allow(unused)]
impl DataRecorder for SimpleDataRecorder {
    fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState) { 
        self.esp = esp;
    }
    fn poll(&mut self, esp: ElevatorState, dst: u64) {}
    fn summary(&mut self) {}
}


#[allow(unused)]
pub fn simulate_elevator<'a, MC: MotorController, DR: DataRecorder>(
    esp: ElevatorSpecification, // Copy/ref in 
    est: ElevatorState, 
    req: FloorRequests, 
    mc: &mut MC, 
    dr: &'a mut DR
) -> &'a DR { // ref out as immutable
    // Implements and modifies borrow-refs here...      

    // Immutable input becomes mutable local state (defensive copies).
    // Does not modify the external data. 
    let mut esp = esp.clone(); // Copies esp input with a deep copy (own new copy)
    let mut est = est.clone(); // (own new copy); does not take (est exists outside still)
    let mut req = req.clone(); // (own new copy); does not take (req exists outside still) 
    let floor_requests: FloorRequests = vec![1, 2, 3];

    // Lazy implement here (not fully done, just demoing usages)
    // Modifying only the copies here (zero out)
    esp.floor_count = 0;
    esp.floor_height = 0.0;
    esp.carriage_weight = 0.0;
    est.location = 0.0;
    floor_requests.iter().for_each(|flr| req.push(*flr));
    // Previous settings now modifed

    // Initialize MotorController and DataController
    mc.init(esp.clone(), est.clone()); // Copy into ref
    dr.init(esp.clone(), est.clone()); // Copy into ref

    dr
}