use crate::accounts::account::Account;

pub struct DebtAccount {
    account: Account
}

impl DebtAccount {
    pub fn new(name: &str, balance: f64, interest_rate: f64) -> DebtAccount {
        let interest_rate: f64 = interest_rate / 100.0;

        let new_account: Account = Account {name: name.to_string(), balance, interest_rate};

        DebtAccount {
            account: new_account
        }
    }

    pub fn to_string(&self) -> String {
        format!("Name: {}, Balance: {:.2}, Interest Rate: {:.2}%", self.account.name, self.account.balance, self.account.interest_rate)
    }

    pub fn calculate_payoff_time(&self, monthly_payment: f64) -> u32 {
   
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

}