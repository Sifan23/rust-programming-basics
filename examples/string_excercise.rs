#![allow(unused)]

pub fn hello() -> String {
    let s: String = String::from("Hello Rust");
    s
}

pub fn greet(name: &str) -> String {
    let t = format!("Hello {}", name);
    t;
    let mut s = "Hello".to_string();
    s+=" ";
    s+=&name;
    s
    
}

pub fn append(mut s: String) -> String {
 s.to_string();
 s+=" ";
 s+="World";
 s+=" ";
 s+="!"; 
 s
}

fn main() {
    let s = hello();
   println!("s: {}", s);
   let s = greet("name");
   println!("s: {}", s);
   let s = append("Hello".to_string());
   println!("s: {}", s);

}