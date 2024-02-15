pub struct Account {
    pub(crate) name: String,
    pub(crate) balance: f64,
    pub(crate) interest_rate: f64
}

pub trait Accountable {
    fn new(name: &str, balance: f64, interest_rate: f64) -> Self;
    fn get_name(&self) -> &str;
    fn get_balance(&self) -> f64;
    fn get_interest_rate(&self) -> f64;
    
    fn to_string(&self) -> String;
    
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn calculate_interest(&self) -> f64;

    fn calculate_future_balance(&self, monthly_payment: f64, months: u32) -> f64;
}

// pub trait Account {
//     type Name;
//     type Balance;
//     type InterestRate;
// }