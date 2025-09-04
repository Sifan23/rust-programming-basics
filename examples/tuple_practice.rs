#![allow(unused)]

fn return_many() -> (i32, bool) {
    (42, true)
} 

fn no_return(){}

fn return_empty_tuple() -> (){}

//Tuples - fixed size, mixed types, known at compile time
fn main() {
    let t: (bool, char, u32) = (true, 'a', 42);
    println!("{} ,{} ,{}", t.0, t.1, t.2);
    //Empty tuple
    let t = ();
    //Result<(), String> = Ok(()) | Err("");
    //Nested tuple
    let nested = (('a', 1.23), (true, 1u32, -1i32), ());
    println!("nested.0.1 {}",(nested.1).2);
    //Destructuring a tuple
    let t: (bool, char, u32) = (true, 'a', 42);
    let (a, b, c) = t;
    println!("a: {} ,b: {} ,c: {}", a, b, c);
    let (_, _, c) = t;
    println!("c: {}", c);
    //Function that return multiple values using tuple
    let(a,b) = return_many();
    println!("a: {}, b: {}", a, b);
}