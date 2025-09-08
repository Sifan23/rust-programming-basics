#![allow(unused)]
#[derive(Debug, PartialEq)]

pub struct Account {
    address: String,
    balance:u32,
}

pub fn new(address: String) -> Account {
    Account {
        address,
        balance: 0
    }
}

fn main() {
    println!("{:?}", new("0x1231534".to_string()));
}