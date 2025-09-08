pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
    let v = vec![x,y,z];
    v
}


fn main () {
    let v: Vec<u32> = vec![1,2,3];
    println!("Vector: {:?}", v);
}