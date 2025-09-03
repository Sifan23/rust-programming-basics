#![allow(unused)]

fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y ;
}

fn add (x: u32, y: u32) -> u32 {
   
    x + y 
}

fn mul (x: u32, y: u32) -> u32 {
    x * y
}

fn div (x: f64, y: f64) -> f64  {
    x / y
}

fn main () {
     let x = 2;
     let y = 3;
     let z = add(x, y);
        println!("The sum of {} and {} is {}", x, y, z);
    let z = mul(x,y);
        println!("The multiplication of {} and {} is {}", x,y,z);
    let z = div(x.into(),y.into());
        println!("The division of {} and {} is {}", x,y,z);
}