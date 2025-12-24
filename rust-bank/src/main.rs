

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, balance: i32, holder: impl Into<String>) -> Self {
        Self { 
            id,
            balance,
            holder: holder.into(),
         }
    }
}

#[derive(Debug)]
struct Bank {
    next_id: u32,
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Self { next_id: 1, accounts: Vec::new() }
    }

    fn add_account(&mut self, balance: i32, holder: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.accounts.push(Account::new(id, balance, holder));

        id
    }

    fn get_by_id(&self, id: u32) -> Option<&Account> {
        self.accounts.iter().find(|a| a.id == id)
    }

    fn get_by_id_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.id == id)
    }

    fn print_one(&self, id: u32) -> Result<(), String> {
        let account = self.get_by_id(id).ok_or_else(|| format!("No account with id {id}"))?;
        println!("Account status: {:#?}", account);
        Ok(())
    }

    fn print_all(&self) {
        for account in &self.accounts {
            println!("Account status: {:#?}", account);
        }
    }

    fn deposit(&mut self, id: u32, amount: i32) -> Result<(), String> {
        if amount < 0 {
            return Err("amount must be >= 0".into());
        }
        let acct = self.get_by_id_mut(id).ok_or_else(|| format!("No account with id {id}"))?;
        acct.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, id: u32, amount: i32) -> Result<(), String> {
        if amount < 0 {
            return Err("amount must be >= 0".into());
        }
        let acct = self.get_by_id_mut(id).ok_or_else(|| format!("No account with id {id}"))?;
        if acct.balance < amount {
            return Err("insufficient funds".into());
        }
        acct.balance -= amount;
        Ok(())
    }

    fn print_balance(&self, id: u32) {

    }

}

#[derive(Debug)]
struct PhonyAccount {
    balance: i32,
}

#[test]
fn test_account_init() {
    let accnt = Account::new(1, 0, String::from("Dan"));
    assert_eq!(accnt.balance, 0);
}

#[test]
fn test_bank_add_account_ids_increment() {
    let mut bank = Bank::new();
    let id1 = bank.add_account(0, "Dan");
    let id2 = bank.add_account(100, "Steve");
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
}

#[test]
fn test_update_account_deposit_withdraw() {
    let mut bank = Bank::new();
    let id = bank.add_account(1000, "Dan");

    bank.deposit(id, 500).unwrap();
    assert_eq!(bank.get_by_id(id).unwrap().balance, 1500);

    bank.withdraw(id, 200).unwrap();
    assert_eq!(bank.get_by_id(id).unwrap().balance, 1300);

    assert!(bank.withdraw(id, 99999).is_err());
}

fn main() {
    println!("Welcome to the Bank!");
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
    bank.print_all();

    bank.withdraw(2, 300);
    println!("Tony withdrew $300");
    bank.print_one(2); 
}
