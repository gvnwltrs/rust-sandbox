/* rust-main::lib.rs */

use rand::prelude::*;
use core::fmt::Write;

#[allow(unused)]
use std::io::Error;

#[allow(unused)]
use core::fmt::Result;

#[allow(unused)]
use chrono::{Local, Utc};

#[derive(Debug)]
#[allow(unused)]
pub struct Assignments {
    x: Option<char>,
    y: Option<char>,
    value: Option<i32>,
}

impl Assignments {
    pub fn new() -> Self {
        Self { 
            x: None,
            y: None,
            value: None,
        }
    }

    pub fn one(&mut self, a: char, val: i32) {
        self.x = Some(a);
        self.value = Some(val);
    }
}

// 0) Three core discplines of Rust and in programming generally

// 1) Variables and mutability

// Why? Shadowing allows you to change the type that a var name 
// holds, but still allowing for a reuse of the name without having
// to do somethign like a typecast. Think healthier alternative to typecasting. 
pub fn shadowing() -> Assignments {
    let _val = "?";
    let _val = 2.0;
    let _val = 3;
    let mut a = Assignments::new(); 
    a.one('a', _val);
    a
} 

pub fn borrow_checker() {
    let mut x = Box::new(42); // heap allocated; think of Box == smart_pointer
    let r = &x; // 'a -- we know that x lives outside of this slot

    if rand::rng().random::<f64>() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r); // 'a
    }
}

// Trying write formats
pub fn write_fmt_to_buf<W: Write>(_in: i32, out: &mut W) -> core::fmt::Result {
    // unimplemented!();
    write!(out, "value={}...", _in)
}

pub fn updating_a_variable() {
    println!("let x = 1;");
    let _x = 1;
    println!("We can update x by shadowing with: let x = 2;");
    let _x = 2;
    println!("But we cannot change x otherwise since it is immutable with: let x =");
    println!("We could rewrite x however, by letting it be mutable: let mut x = 0;");
    let mut _x = 0;
    println!("Now we can update x: x = 1;");
    _x = 1;
    let _ = _x;
    println!("\n");
}

pub fn set_a_constant() {
    println!("We can set a constant by: const X: u32 = 0;");
    const _X: u32 = 0;
    println!("
        But this is different than variables because we 
        MUST use a type annotation such as: const X: u32"
    );

    println!("\n");
}

pub fn performing_shadowing() {
    println!("We can perform shadowing by simply re-assigning a previously used variable alias:
        let x = 0;
        let x = 1;\n"
    );
    let _x = 0;
    let _x = 1;
    println!("In the previous case, it was a little pointless since we are using the same type.");
    println!("To gain the real value from shadowing, we can use it as an alternative to typecasting:
        let x = \"change to a string\";\n"
    );
    println!("Now x has gone from an integer to a string type (&str more precisely)\n");

    println!("\n");
}

// 2) Data types


// 3) Functions

// Math expressions
pub fn add_expressions(a: i32, b: i32) -> i32 {
    println!("We can do an expression by: let out = {{ a + b }};");
    let _out = { a + b }; // is it preferrable to use braces to signal what is an expression?
    println!("Or we can do it shorthand by: let out = a + b;");
    let out = a + b;
    println!("
        In most cases, whatever you decide to use is a matter of preferrence
        But it seems like the braces way indicates an expression more explicitly...
        "
    );
    println!("\n");
    out
}

// 4) Comments

// 5) Control flow

// Conditional expressions
pub fn conditional_expression(a: i32, b: i32) -> bool {
    println!("
        We can do common conditional expressions using if, else if, and else:
            if a == b {{
                println!(\"a is equal to b\");
            }} else if a < b {{
                    println!(\"a is less than b\");
            }} else {{
                println!(\"a is not greater than b\");
            }}"
    );
    if a == b {
        println!("a is equal to b");
        true
    } else if a < b {
        println!("a is less than b");
        true
    } else {
        println!("a is not greater than b");
        false
    }
}

// 6) Loops

pub fn wrap_around_conditional(start: i32) -> bool {
    println!("Handling multiple conditionals with if-else.");
    if start < 3 { println!("Come on...{:#?}", start); return false};
    let mut count = 1;
    let begin = (start+1) % start ;
    let mid = (start/2) % start;
    let end = (start-1) % start;
    while count > 0 {
        if count % start == begin {
            println!("Currently at: {:#?}", count);
        } else if count % start == mid {
            println!("Getting closer: {:#?}", count);
        } else if count % start == end {
            println!("We made it! {:#?}", count+1);
            count = 0;
            continue;
        } else {
            println!("We're somewhere: {:#?}", count);
        }  
        count += 1;
    }

    true
}

pub fn if_let(num: i32) -> bool {
    let _case = if num == 1 {true} else{false};
    _case
}

pub fn conditional_loop(num: i32) -> i32 {
    println!("REMEMBER: While loops are great counters or countdowns. Not for collections though...");
    println!("Also, even for countdowns or counters, a for loop might be a better option since it sets clear boundaries...");
    let mut counter = num;
    while counter != 0 {
        println!("{:#?}", counter);
        counter -= 1;
    }
    counter
}

// 7) Ownership 

pub fn where_does_this_string_live(string: &str) -> &'static str {
    let mut _string = string;
    let mut buffer = [0u8; 32];
    if _string == "" {
        let text = "stack-text";
        let len = text.len();
        buffer[..len].copy_from_slice(text.as_bytes());
        _string = std::str::from_utf8(&buffer[..len]).unwrap();
    };

    let stack_val = 0;
    let stack_addr = &stack_val as *const _ as usize;
    let heap_addr = Box::into_raw(Box::new(0)) as usize;
    let literal_addr = _string.as_ptr() as usize;
    let f_addr = where_does_this_string_live as *const () as usize;

    let dist_to_f = (literal_addr as isize - f_addr as isize).abs();
    let dist_to_stack = (literal_addr as isize - stack_addr as isize).abs();
    let dist_to_heap = (literal_addr as  isize - heap_addr as isize).abs();

    if  dist_to_stack < dist_to_f && dist_to_stack < dist_to_heap {
       "stack"
    } else if dist_to_f < dist_to_heap {
        ".rodata (near fn)"
    } else {
        "heap"
    }
}

// 8) Ownership & functions 

pub fn takes_ownership(s: String) {
    let _s = s;
}

pub fn makes_copy(val: i32) -> i32 {
    let _val = val;
    _val
}

// 9) Borrowing & references

pub fn gives_ownership() -> String {
    String::from("Here, have a string. You own it!")
}

// 10) Mutable references

pub fn mutate_reference(s: &mut String) {
    s.push_str("and now we have this...");
}

// 11) Dangling references

// Slice types 
pub fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[derive(Debug)]
#[allow(unused)]
pub struct MyStruct {
    x: i32,
    y: i32,
}

#[derive(Debug)]
#[allow(unused)]
pub struct TupleStruct(i32, i32);

pub fn init_struct() -> MyStruct {
    MyStruct { x: 42, y: 43 }
}

pub fn init_tup_struct(x: i32, y: i32) -> TupleStruct {
    TupleStruct(x, y)
}

#[derive(Debug)]
#[allow(unused)]
pub struct MyData{
    values: i32,
}

impl MyData {
    pub fn new() -> Self {
        Self {
            values: 42,
        }
    }

    pub fn reset(&mut self) {
        self.values = 42;
    }
}

pub fn using_data_method(data: &mut MyData) {
    data.values = 100;
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Vault {
    Name(String),
    ID(i32),
}

pub fn update_vault(vault: &mut Vault, val: &Vault) {
    *vault = val.clone();
} 

// Returns by giving ownership to a string (move)
pub fn read_vault(vault: &Vault) -> String {
    let mut msg = String::from("Type:"); 

    match vault {
        Vault::Name(name) => { 
            let variant = String::from(" String, ");
            msg.push_str(variant.as_str());
            msg.push_str(name)
        },
        Vault::ID(id) => {
            let variant = String::from(" i32, ");
            msg.push_str(variant.as_str());
            msg.push_str(id.to_string().as_str());
        }
    }
    msg
}

// Exploring "kinds" with enums 
#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Operation {
    Quit(String), // Contains a "quit" message
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn execute_op(op: &Operation) -> String {
    let mut msg = String::default();
    match op {
        Operation::Quit(_) => {println!("Quit operation executing.");}
        Operation::Move { .. } => { println!("Move operation executing."); }
        Operation::Write(_msg) => { 
            println!("Write operation executing."); 
            msg.push_str(_msg); 
        }
        Operation::ChangeColor(_, _, _) => { println!("Change color operation executing."); }
    }
    msg
}
// Using braces {} for match arms—even when they only contain a single expression—is a
// common practice: it makes the code consistent and much easier to extend later. Since
// the match is already exhaustive, it just needs to tidy up the unreachable code to make 
// the compiler happy. 

// Experimenting with options

#[derive(Debug)]
pub enum SelectOption {
    First,
    New,
    Last,
} 

pub fn get_data(s: &SelectOption) -> bool {
    match s {
        SelectOption::First => false,
        SelectOption::New => true,
        SelectOption::Last => false,
    }
}

#[derive(Debug, Clone)]
pub enum UsState {
    Alabama,
    Michigan,
}

#[derive(Debug, Clone)]
pub struct CoinCollection {
    pub collection: Vec<Coin>
}

impl CoinCollection {
    pub fn default() -> Self {
        Self {
            collection: Vec::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn add_coin_to_collection(c: &mut CoinCollection, entry: &Coin) {
    c.collection.push(entry.clone());
}

pub fn value_in_cents(coin: &Coin) -> (String, u8) {
    match coin {
        Coin::Penny => (format!("Penny"), 1),
        Coin::Nickel => (format!("Nickel"), 5),
        Coin::Dime => (format!("Dime"), 10),
        Coin::Quarter(state) => (format!("Quarter, State: {:?}", state), 25),
    }
}

pub fn match_to_condition(o: Option<u32>) -> Option<u32> {
    match o {
        Some(num) => Some(num),
        None => None,
    }
}

pub fn dummy_qualification(t: bool) -> bool {
    match t { true => true, false => false }
}

pub fn match_with_table(num: i32) -> bool {
    match num {
        1 => dummy_qualification(true),
        2 => dummy_qualification(true),
        3 => dummy_qualification(true),  
        _ => dummy_qualification(false)
    }
}

pub enum Entity {
    Android,
    Linux,
    Apple,
    Microsoft,
}

pub fn lookup_callback(arm: Entity) -> Option<String> {
    match arm {
        Entity::Android => Some(String::from("Device: Android")),
        Entity::Linux => Some(String::from("Device: Linux")),
        Entity::Apple => Some(String::from("Device: Apple")),
        Entity::Microsoft => Some(String::from("Device: Microsoft")),
    } 
}

pub fn lookup_table_match(ent: Entity) -> Option<String> {
    let matched_value = lookup_callback(ent);
    match Some(matched_value) {
        Some(val) => val,
        _ => None
    }
}

/*============================================================================== */

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Ready,
	Good,
	Working,
	Success,
	Error,
	Degraded,
	Warning(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThermostatEvent {
	PowerOn,
    Setpoint(f64),
	Shutdown,
	Awaiting,
	Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveEvent {
	Processing,
	Running,
	Inactive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Units {
	Farenheit,
	Celsius,
	Kelvins,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThermostatDataPoint {
	timestamp: Option<String>,
	temperature: Option<f64>,
	setpoint: Option<f64>, 
	trigger_event: ThermostatEvent,
	active_event: ActiveEvent,
	units: Units,
}

/* Actions */
fn gen_timestamp() -> Option<String> {
    Some(Local::now().format("%Y-%m-%dT%H:%M").to_string())
}

pub fn gen_thermo_instance() -> (ThermostatDataPoint, Status) {
    (
        ThermostatDataPoint {
            timestamp: gen_timestamp(),
            temperature: None,
            setpoint: None,
            trigger_event: ThermostatEvent::Awaiting,
            active_event: ActiveEvent::Inactive,
            units: Units::Farenheit, // DEFAULT
        },
        Status::Success
    )
}

pub fn check_status(device: &ThermostatDataPoint) -> Status {
    match device.trigger_event {
        ThermostatEvent::PowerOn => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Error,
                ActiveEvent::Processing => Status::Working,
                ActiveEvent::Running => Status::Error,
            }
        },
        ThermostatEvent::Setpoint(_) => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Error,
                ActiveEvent::Processing => Status::Working,
                ActiveEvent::Running => Status::Error,
            }
        },
        ThermostatEvent::Shutdown => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Success,
                ActiveEvent::Processing => Status::Working,
                ActiveEvent::Running => Status::Error,
            }
        }
        ThermostatEvent::Awaiting => { 
            match device.active_event { 
                ActiveEvent::Inactive => Status::Ready,
                ActiveEvent::Processing => Status::Error,
                ActiveEvent::Running => Status::Good,
            }
        },
        ThermostatEvent::Error => {
            match device.active_event {
                ActiveEvent::Inactive => Status::Error,
                ActiveEvent::Processing => Status::Degraded,
                ActiveEvent::Running => Status::Warning(String::from("Msg: Running in bad state.")),
            }
        }, 
    }
}

fn fake_power_on_device(_sp: Option<f64>) -> (Status, f64) {
   // Fake implementation 
   let fake_reading = 65.0;
   (Status::Good, fake_reading)
}

pub fn init_device(device: &mut ThermostatDataPoint) -> (ThermostatDataPoint, Status) {
    let mut _device = device.clone();
    const DEFAULT_TEMP: f64 = 68.0;
    _device.timestamp = gen_timestamp();
    _device.trigger_event = ThermostatEvent::PowerOn;
    _device.active_event = ActiveEvent::Processing;
    let set_on = Some(DEFAULT_TEMP); 
    _device.setpoint = set_on;
    let set_on: (Status, f64) = fake_power_on_device(set_on);
    match set_on.0 {
        Status::Good => { 
            _device.trigger_event = ThermostatEvent::Awaiting;
            _device.active_event = ActiveEvent::Running;
            _device.temperature = Some(set_on.1);
            (_device, Status::Success)
        },
        _ => { 
            _device.trigger_event = ThermostatEvent::Error;
            _device.active_event = ActiveEvent::Inactive;
            (_device, Status::Error)
        }
    }
}

fn fake_set_temperature_on_device(_device: &mut ThermostatDataPoint) -> Status {
    _device.temperature = Some(65.14);
    Status::Success
}

fn modify_temp_setpoint(device: &mut ThermostatDataPoint, temp: f64) -> (ThermostatDataPoint, Status) {
    let mut _device = device.clone();
    _device.timestamp = gen_timestamp();
    match device.trigger_event {
        ThermostatEvent::Awaiting => {
            _device.trigger_event = ThermostatEvent::Setpoint(temp);
            _device.active_event = ActiveEvent::Processing;
            match fake_set_temperature_on_device(&mut _device) {
                Status::Success => {
                    _device.setpoint = Some(temp);
                    _device.trigger_event = ThermostatEvent::Awaiting;
                    _device.active_event = ActiveEvent::Running;
                    return (_device, Status::Success)
                },
                _ => return (_device, Status::Error)
            };
        },
        _ => {
            _device.trigger_event = ThermostatEvent::Error;
            _device.active_event = ActiveEvent::Running; // doesn't set, continues running
            return (_device, Status::Error)
        }
    }
}

pub fn set_operation(device: &mut ThermostatDataPoint, cfg: &ThermostatEvent) -> (ThermostatDataPoint, Status) {
    let mut _device = device.clone();
    _device.timestamp = gen_timestamp();
    // let mut _device = device.clone();
    let msg = format!("Msg: Not a configurable operation -> ");
    match cfg {
        ThermostatEvent::PowerOn => init_device(&mut _device),
        ThermostatEvent::Setpoint(temp) => { 
            modify_temp_setpoint(&mut _device, *temp)
        },
        invalid_operation => {
            let operation = format!("{:#?}", invalid_operation);
            let msg = msg + &operation[..];
            (_device, Status::Warning(msg))
        }
    }
}


#[cfg(test)]
mod rust_main_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_sanity() {
        assert!(true);
    }

    // NOTE: Try write operation.  
    #[test]
    fn test_execute_write_operation() {
        let write = Operation::Write(String::from("Append"));
        let write_executed = execute_op(&write);
        let expected = String::from("Append");

        assert_eq!(write_executed, expected);
    }

    // NOTE: Try using options to result in single enum variant for condition.
    #[test]
    fn test_enum_option_for_last_item() {
        let option = SelectOption::Last;
        let data_access = get_data(&option);
        let expected = false;

        assert_eq!(data_access, expected);
    }

    // NOTE: Try to get the value of a particular coin. 
    #[test]
    fn test_coin_valuation_for_penny() {
        let penny = Coin::Penny;
        let mut coins = CoinCollection::default();
        add_coin_to_collection(&mut coins, &penny);
        let coin_value = value_in_cents(&penny);
        let expected: (String, u8) = (String::from("Penny"), 1);

        assert_eq!(coin_value, expected);
    }

    #[test]
    fn test_coin_valuation_for_nickel() {
        let nickel = Coin::Nickel;
        let mut coins = CoinCollection::default();
        add_coin_to_collection(&mut coins, &nickel);
        let coin_value = value_in_cents(&nickel);
        let expected: (String, u8) = (String::from("Nickel"), 5);

        assert_eq!(coin_value, expected);
    }

    #[test]
    fn test_coin_valuation_for_dime() {
        let dime = Coin::Dime;
        let mut coins = CoinCollection::default();
        add_coin_to_collection(&mut coins, &dime);
        let coin_value = value_in_cents(&dime);
        let expected: (String, u8) = (String::from("Dime"), 10);

        assert_eq!(coin_value, expected);
    }

    #[test]
    fn test_coin_valuation_for_quarter() {
        let michigan = UsState::Michigan;
        let quarter = Coin::Quarter(michigan);
        let mut coins = CoinCollection::default();
        add_coin_to_collection(&mut coins, &quarter);
        let coin_value = value_in_cents(&quarter);
        let expected: (String, u8) = (String::from("Quarter, State: Michigan"), 25);

        assert_eq!(coin_value, expected);
    }

    #[test]
    fn test_exhaustive_match_pattern() {
        let x: Option<u32> = Some(5);
        let pattern_matched = match_to_condition(x);
        let expected = Some(5);
        assert_eq!(pattern_matched, expected);

        let x: Option<u32> = None; 
        let pattern_matched = match_to_condition(x);
        assert!(pattern_matched != expected);
    }

    #[test]
    fn test_match_with_table() {
        let table_match = match_with_table(1);
        let expected = true;
        assert_eq!(table_match, expected);
    }

    #[test]
    fn test_lookup_table() {
        let query = Entity::Android;
        let found = lookup_table_match(query);
        let expected = Some(String::from("Device: Android"));
        assert_eq!(found, expected);
    }

    /* ===================================================================== */
    // 1. Power on stage
    #[test]
    fn test_device_check_status_handles_defaults() {
        let thermo_instance = gen_thermo_instance(); 
        let status = check_status(&thermo_instance.0);
        let expected = Status::Ready;
        assert_eq!(status, expected);
    }

    #[test]
    fn test_boot_thermostat_power_on_good() {
        let mut thermo_instance = gen_thermo_instance(); 
        let initialized = init_device(&mut thermo_instance.0);
        let expected = Status::Success; 
        assert_eq!(initialized.1, expected);
    }
	
    // 2. Set operation
    #[test]
    fn test_configure_new_temp_setpoint() {
        let mut temp_device = gen_thermo_instance();
        let status = check_status(&temp_device.0);
        let expected = Status::Ready;
        assert_eq!(status, expected);
        
        let config = ThermostatEvent::Setpoint(68.0);
        let operation_set = set_operation(&mut temp_device.0, &config);
        let expected = Status::Success;
        assert_eq!(operation_set.1, expected);
    }

}