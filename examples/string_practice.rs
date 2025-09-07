#![allow(unused)]

// String and &str 
// use String for ownership & mutable
// use &str for read only & sring literal
fn main() {
    let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let length: usize = msg.len();
    let slice: &str = &msg[..5];
    println!("slice: {}", slice);

    let s: &str = "Hello Rust";
    let t: String = s.to_string();


    //Rust automatically converts &String to &str
    let s: String = String::from("Hello Rust1");
    print(&s);

    let s: &str = "Hello Rust2";
    print(s);

    //Append &str to String
    let mut s: String = String::from("Hello Rust");
    s+=" ";
    s+= "World";
    println!("s: {}", s);

    //String interpolation - format!
    let lang = "Rust";
    let emoji = "🦀";
    let mut s = "Hello".to_string();
    s+=" ";
    s+=lang;
    s+="";
    s+=emoji;
    println!("s: {}", s);
    let s = format!("Hello {} {}", lang, emoji);
    println!("s: {}", s);
}

fn print(s: &str) {
    println!("s: {}", s);
}