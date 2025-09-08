#![allow(unused)]



fn main () {
    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3); 

    println!("Vector: {:?}", v);

    let v: Vec<u32> = vec![1,2,3];
    let v = vec![1u8,2,3]; 

    //vector from 0 to 100
    let v: Vec<i8> = vec![0i8; 100];
    println!("Vector: {:?}", v);

    //Get
    println!("{:?}", v[1]);

    //Option<&i8>
    //Index valid => Some(&i8)
    //Index invalid => None
    println!("v[1]: {:?}", v.get(1));
    println!("v[1000]: {:?}", v.get(1000));

    //update
    let mut v = vec![1,2,3];
    v[1] = 4;
    println!("v: {:?}", v);

    //pop - remove the last element
    let last: Option<u32> = v.pop();
    println!("Last: {:?}", last);
    println!("v: {:?}", v);

    //slice
    let v = vec![1,2,3,4,5];
    let s = &v[1..3];
    println!("Slice: {:?}",s);

}