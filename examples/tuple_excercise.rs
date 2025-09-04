pub fn first(t: (bool, u32, char)) -> bool {
   t.0
}

pub fn last(t: (bool, u32, char)) -> char {
    t.2
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    (t.1, t.0)
}

pub fn main() {
    let tuple = (true, 42u32, 'a');;
    println!("First element: {}", first(tuple));
    println!("Last element: {}", last(tuple));
    println!("Swapped tuple: {:?}", swap((1, 2)));

}

