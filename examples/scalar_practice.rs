#![allow(unused)]


//Scalar types are the most basic types in Rust. They represent a single value and include types like integers, floating-point numbers, booleans, and characters.
fn main() {

    //Signed Integers
    // Range: -2^(n-1) to 2^(n-1)-1
    let a: i8 = -1; // 8-bit signed integer
    let b: i16 = 2; // 16-bit signed integer
    let c: i32 = 3; // 32-bit signed integer
    let d: i64 = -4; // 64-bit signed integer
    let e: i128 = 5;
    //Unsigned Integers
    // Range: 0 to 2^n - 1
    let f: u8 = 1; // 8-bit unsigned integer
    let g: u16 = 2; // 16-bit unsigned integer
    let h: u32 = 3; // 32-bit unsigned integer
    let i: u64 = 4; // 64-bit unsigned integer
    let j: u128 = 5;
    //Depends on the architecture of the computer
    let k: isize = -1; // Signed integer with the same number of bits as the platform's pointer type
    let l: usize = 1; // Unsigned integer with the same number of bits as the platform's pointer type
    //Floating-Point Numbers
    let m: f32 = 3.14; // 32-bit floating-point number
    let n: f64 = 2.718281828459045; // 64
    //Boolean
    let o: bool = true; // Boolean type
    let p: bool = false; // Boolean type
    //Character
    let q: char = 'a'; // Character type
    let r: char = '😊'; // Unicode character
    //Type Conversion
    let s: i32 = -1;
    let t: u32 = s as u32; // Convert i32 to f64
    println!("{s} as u32 = {t}");
    //Min and Max Values
    let i_max = i32::MAX;
    let i_min = i32::MIN;
    println!("i32 max: {}, i32 min: {}", i_max, i_min);
}