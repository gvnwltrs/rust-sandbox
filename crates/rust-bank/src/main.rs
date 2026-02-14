use rust_bank::*;

fn main() {
    println!("=====Welcome to the Bank!=====\n");
    let mut bank = Bank::new(); 
    let mut phony : Vec<PhonyAccount> = vec![
        PhonyAccount { balance: 0 }, 
        PhonyAccount { balance: 10 },
    ];

    match phony.first_mut() {
        Some(phony) => {
            phony.balance += 1;
            println!("{:#?}", phony);
        },
        None => println!("No account found")
    }

    bank.add_account(30000, String::from("George M"));
    println!("Added account.");

    bank.add_account(20000, String::from("Tony D"));
    println!("Added account.");

    println!("Checking all account statuses...");
    bank.msg_all();

    let withdraw = bank.withdraw(2, 300);
    println!("Tony withdrew $300? {:#?}", withdraw.is_ok());
    let msg = bank.msg_one(2); 
    println!("withdraw okay? {:#?}", msg.is_ok());

    let deposit = bank.deposit(2, 100);
    println!("Tony deposit $100? {:#?}", deposit.is_ok());

    println!("Balance: {:#?}", bank.see_balance());

}
