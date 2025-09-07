#![allow(unused)]

fn main() {
    //Array
    let arr: [u32; 3] = [1,2,3];
    println!("Array1: {}", arr[2]);
    //Write
    let mut arr: [u32; 3] = [1,2,3];
    arr[1] = 4;
    println!("Array2: {}", arr[1]);
    let mut arr: [u32; 10] = [0; 10];
    println!("arr: {:?}", arr);
    //Slice
    let nums: [u32; 5] = [1,2,3,4,5];
    //First
    let first: &[u32] = &nums[..2];
    println!("First two: {:?}", first);
    //Last
    let last : &[u32] =  &nums[2..];
    println!("Last three: {:?}", last);
    //Middle
    let middle : &[u32] = &nums[1..2];
    println!("Middle two: {:?}", middle);
}