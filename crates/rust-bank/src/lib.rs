
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    pub fn new(id: u32, balance: i32, holder: impl Into<String>) -> Self {
        Self { 
            id,
            balance,
            holder: holder.into(),
         }
    }
}

#[derive(Debug)]
pub struct Bank {
    next_id: u32,
    accounts: Vec<Account>,
}

impl Bank {
    pub fn new() -> Self {
        Self { next_id: 1, accounts: Vec::new() }
    }

    pub fn add_account(&mut self, balance: i32, holder: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.accounts.push(Account::new(id, balance, holder));

        id
    }

    pub fn get_by_id(&self, id: u32) -> Option<&Account> {
        self.accounts.iter().find(|a| a.id == id)
    }

    pub fn get_by_id_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.id == id)
    }

    pub fn print_one(&self, id: u32) -> Result<(), String> {
        let account = self.get_by_id(id).ok_or_else(|| format!("No account with id {id}"))?;
        println!("Account status: {:#?}", account);
        Ok(())
    }

    pub fn print_all(&self) {
        for account in &self.accounts {
            println!("Account status: {:#?}", account);
        }
    }

    pub fn deposit(&mut self, id: u32, amount: i32) -> Result<(), String> {
        if amount < 0 {
            return Err("amount must be >= 0".into());
        }
        let acct = self.get_by_id_mut(id).ok_or_else(|| format!("No account with id {id}"))?;
        acct.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, id: u32, amount: i32) -> Result<(), String> {
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

    pub fn see_balance(&self) -> Vec<Account> {
        self.accounts
            .iter()
            .cloned()
            .collect()
    }

}

#[derive(Debug)]
#[allow(unused)]
pub struct PhonyAccount {
    pub balance: i32,
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

#[test]
fn test_see_balance() {
    let mut bank = Bank::new(); 
    bank.add_account(100, "Gergio");
    bank.add_account(0, "Marco");
    bank.add_account(1000, "Goldberg");

    let balances = bank.see_balance();
    let expected = vec![
        Account {
            id: 1,
            balance: 100,
            holder: String::from("Gergio"),
        },
        Account {
            id: 2,
            balance: 0,
            holder: String::from("Marco"),
        },
        Account {
            id: 3,
            balance: 1000,
            holder: String::from("Goldberg"),
        }, 
    ];

    assert_eq!(balances.get(0), expected.get(0));
}