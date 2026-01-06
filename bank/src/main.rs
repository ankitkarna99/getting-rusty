#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, balance: i32, holder: String) -> Self {
        Account {
            id,
            balance,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        return self.balance;
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;

        return self.balance;
    }
    
    fn summary(&self) -> String {
        return format!("{} [{}] has a balance of {}", self.holder, self.id, self.balance);
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: vec![],
        }
    }

    fn total_balance(&self) -> i32 {
        return self.accounts.iter().map(|account| account.balance).sum();
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn summary(&self) -> Vec<String> {
        return self.accounts.iter().map(|account| account.summary()).collect::<Vec<String>>();
    }
}

fn main() {
    let mut bank = Bank::new();

    let mut account = Account::new(1, 0 , String::from("Ankit Kumar Karna"));

    account.deposit(500);
    account.withdraw(200);

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
