use crate::accounts::account::Account;
use crate::accounts::account::Accountable;

pub struct DebtAccount {
    account: Account
}

impl Accountable for DebtAccount {
    
    fn new(name: &str, balance: f64, interest_rate: f64) -> DebtAccount {
        let interest_rate: f64 = interest_rate / 100.0;

        let new_account: Account = Account {name: name.to_string(), balance, interest_rate};

        DebtAccount {
            account: new_account
        }
    }

    fn get_balance(&self) -> f64 {
        return self.account.balance;
    }

    fn get_name(&self) -> &str {
        return self.account.name.as_str()
    }

    fn get_interest_rate(&self) -> f64 {
        return self.account.interest_rate;
    }

    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Invalid deposit amount".to_string());
        }
        self.account.balance += amount;
        Ok(())
    }

    fn to_string(&self) -> String {
        format!("Name: {}, Balance: {:.2}, Interest Rate: {:.2}%", self.account.name, self.account.balance, self.account.interest_rate)
    }

    fn calculate_future_balance(&self, monthly_payment: f64, months: u32) -> f64 {
   
        let monthly_interest_rate:f64 = (self.account.interest_rate / 12.0).into();
        
        // println!("Annual interest {}", self.interest_rate);
        // println!("Monthly interest {}", monthly_interest_rate);
        // let mut ending_balance: f64 = self.balance;
        // let mut _months: u32 = 0;
        // while ending_balance > 0.0 {
        //     ending_balance = (ending_balance - monthly_payment) * (1.0 + monthly_interest_rate);
        //     _months = _months + 1;
        // }
        
        let _months: u32 = (f64::ln( 1.0 - ((monthly_interest_rate/monthly_payment) * self.account.balance )) / f64::ln(1.0 + monthly_interest_rate)).abs().ceil() as u32;

        return _months
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        todo!()
    }

    fn calculate_interest(&self) -> f64 {
        todo!()
    }

}