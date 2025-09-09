use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut accounts: HashMap<String, u32> = HashMap::new();
    accounts.insert(address, amount); 
    accounts
}

fn main() {
    let accounts = init("Alice".to_string(), 1000);
    println!("{:?}", accounts);
}
