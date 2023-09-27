trait Account {
    type BalanceType;

    fn deposit(&mut self, amount: Self::BalanceType) -> Result<(), String>;
    fn withdraw(&mut self, amount: Self::BalanceType) -> Result<(), String>;
    fn balance(&mut self) -> Self::BalanceType;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
} 

impl Account for BankAccount {
    type BalanceType = f64;

    fn balance(&mut self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: Self::BalanceType) -> Result<(), String> {
        if amount > 0.0 {
            self.balance += amount;
            Ok(()) 
        } else {
            Err("Amount must be higher than zero".to_string())
        } 
    }

    fn withdraw(&mut self, amount: Self::BalanceType) -> Result<(), String>  {
        if amount > 0.0 {
            if self.balance - amount < 0.0 {
                Err("Not enough funds".to_string())
            } else {
                self.balance -= amount;
                Ok(())
            }
        } else {
            Err("Amount must be higher than zero".to_string())
        } 
    }
}


fn main() {
    let mut acc1 = BankAccount {
        account_number: 1701,
        holder_name: "User1".to_string(),
        balance: 0.0
    };

    let mut acc2 = BankAccount {
        account_number: 1702,
        holder_name: "User2".to_string(),
        balance: 300.0
    };

    match acc1.deposit(5000.0) {
        Ok(_) => {
            println!("Deposit operation to {} done!", acc1.holder_name);
        },
        Err(e) => {
            println!("An error occured! {:?}", e);
        }
    }
    match acc2.withdraw(350.0) {
        Ok(_) => {
            println!("Deposit operation to {} done!", acc2.holder_name);
        },
        Err(e) => {
            println!("An error occurred! {}", e);
        }
    }

    println!("Acc1 Balance: {:?}", acc1.balance());
    println!("Acc2 Balance: {:?}", acc2.balance()); 
}