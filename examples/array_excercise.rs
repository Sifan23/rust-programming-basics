#![allow(unused)]

pub fn zeros() -> [u32; 100] {
    let arr: [u32; 100] = [0; 100];
    return (
        arr
    );
}

pub fn first_3(s: &[u32]) -> &[u32] {
    return &s[..3];
}

pub fn last_3(s: &[u32]) -> &[u32] {
    return &s[s.len()-3..];
}

pub fn middle(s: &[u32]) -> &[u32] {
    return &s[1..s.len()-1];
}

fn main() {
//   println!("{:?}", zeros());
    let arr: [u32; 5] = [1,2,3,4,5];
    println!("{:?}", first_3(&arr));
    println!("{:?}", last_3(&arr));
    println!("{:?}", middle(&arr))
}