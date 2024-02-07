mod accounts;

use accounts::debt::DebtAccount;
use accounts::investment::InvestmentAccount;

fn main() {
    println!("Hi there");
    let debt_account: DebtAccount = DebtAccount::new("Savings", 1000.0, 4.6);
    println!("{}", debt_account.to_string());

    let payment: f64 = 0.0;
    println!("It's going to take {} months to payoff at {}", debt_account.calculate_payoff_time(payment), payment);

    let savings_account: InvestmentAccount = InvestmentAccount::new("Savings", 1000.0, 5.0);
    println!("{}", savings_account.to_string());

    let months: u32 = 15;
    println!("In {} months, the balance will be {} while also paying in {} a month", months, savings_account.calculate_future_balance(payment, months), payment)   
}