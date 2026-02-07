
use rust_collections::*;

fn main() {
    println!("Handling structs");
    let istruct = init_struct();
    println!("Initalizing a struct and returning: {:#?}\n", istruct);
    let tstruct = init_tup_struct(42, 43);
    println!("Initalizing a tuple struct and returning: {:#?}\n", tstruct);

    println!("Using methods");
    let mut data = MyData::new(); // single owner
    println!("Initialized to: {:#?}", data);
    using_data_method(&mut data); 
    println!("After borrowing and modifying: {:#?}\n", data);
    println!(
        "The init was an \"Associated Function\",\
         while the function that called it was a free function\n\
    ");
    println!("Now calling method to reset.");
    data.reset();
    println!("Result for reset: {:#?}\n", data);

    println!("Using enums & structs");
    let mut old_vault = Vault::Name(String::from("Old"));
    println!("Old vault name: {:#?}", old_vault);
    let mut new_vault = Vault::Name(String::from("New"));
    update_vault(&mut old_vault, &new_vault);
    println!("Updated vault name: {:#?}\n", old_vault);
    println!("Now reading from vault: {:#?}\n", read_vault(&old_vault));

    println!("Changing the vault type.");
    new_vault = Vault::ID(42);
    update_vault(&mut old_vault, &new_vault);
    println!("Updated vault type: {:#?}\n", old_vault);

    println!("Now reading from vault: {:#?}\n", read_vault(&new_vault));

    let mut msg = format!("Coin value: ");
    let state = UsState::Michigan;
    let value = value_in_cents(&Coin::Quarter(state));
    msg.push_str(&format!("{:#?}", value));
    // println!("Coin value: {:?}\n", value);
    println!("{:#?}",msg);

    let mut collection = CoinCollection::default();
    let quarter = Coin::Quarter(UsState::Michigan);
    add_coin_to_collection(&mut collection, &quarter);

    let coin_value = value_in_cents(&collection.collection.get(0).unwrap());
    println!("Value: {:?}", coin_value);

    let penny = Coin::Penny;
    let coin_value = value_in_cents(&penny);
    println!("Coin value {:?}", coin_value);
    
    let x = Container { element: 5 }; 
    try_move(x);
    println!("Original: {}", String::from("Element moved.\n")); 

}
