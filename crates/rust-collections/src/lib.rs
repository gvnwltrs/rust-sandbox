/* rust_collections::lib.rs */

#[allow(unused)]
use core::fmt::Write;

#[allow(unused)]
use std::io::Error;

#[allow(unused)]
use core::fmt::Result;

#[allow(unused)]
use chrono::{Local, Utc};

// Data 

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

// Data 

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

// Data 

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

// Using braces {} for match arms—even when they only contain a single expression—is a
// common practice: it makes the code consistent and much easier to extend later. Since
// the match is already exhaustive, it just needs to tidy up the unreachable code to make 
// the compiler happy. 
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

// Data

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

// Data 

#[derive(Debug)]
pub enum SelectOption {
    First,
    New,
    Last,
} 

// Actions

// Pure functions
pub fn get_data(s: &SelectOption) -> bool {
    match s {
        SelectOption::First => false,
        SelectOption::New => true,
        SelectOption::Last => false,
    }
}

// Data 

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

// Actions 

pub fn add_coin_to_collection(c: &mut CoinCollection, entry: &Coin) {
    c.collection.push(entry.clone());
}

// Pure functions

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
pub struct Container { pub element: u32 }

pub fn try_move(u: Container) {
    println!("Value received: {}", u.element);
}

// Tests 

#[cfg(test)]
mod rust_collections_tests {
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

}