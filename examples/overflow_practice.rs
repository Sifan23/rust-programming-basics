#![allow(unused)]

fn main() {
    let mut x = u32::MAX;
    x += 1;
    println!("u32 max : {}, x: {}", u32::MAX, x);

    // u32::checked_add - returns None if overflow occurs
    let x = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", x);
    // u32::wrapping_add - wraps around the value if overflow occurs
    let x = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {}", x);
}