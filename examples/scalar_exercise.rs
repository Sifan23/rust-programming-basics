pub fn eq() {
    let x: char = 'a';
    let y: char = 'b';
    let z: bool = x == y;
    println!("{x} == {y} is {z}");
}

pub fn add() {
 let x: f32 = 1.0;
 let y: f32 = 2.0;
 let z:f32 = 3.0;
 let sum = x + y + z;
 println!("{x} + {y} + {z} = {sum}");
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    let x = x as f32;
    let y = y as f32;       
    println!("{} + {} + {} = {}", x, y, z, x + y + z);
    x + y + z
}

pub fn main() {
    eq();
    add();
    cast(1, -1, 1.0);
}