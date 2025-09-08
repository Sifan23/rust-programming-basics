#![allow(unused)]
#[derive(Debug, PartialEq)]

enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32}
}


//Enum
fn main() {
    let cmd : Command = Command::Play;
    let cmd : Command = Command::Stop;
    let cmd : Command = Command::Resize { width: 100, height: 100};

    println!("{:?}", cmd);

    //PartialEq
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);

    println!("{}", cmd0 == cmd1);

    //Option<T> = Some(T) | None
    let x: Option<i32> = Some(5);
    let x: Option<i32> = None;

    //Result<T, E> = Ok(T) | Err(E)
    // "100" -> 100
    let x: Result<i32, String> = Ok(100);
    //"1234zcxv" -> Error
    let x: Result<i32, String> = Err("Failed to parse".to_string());
    
}