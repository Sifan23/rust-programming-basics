#![allow(unused)]
#[derive(Debug, PartialEq)]

//Struct

struct Point {
    x: i32,
    y: i32
}

struct Point3D (i32, i32, i32);

#[derive(Debug, PartialEq)]
struct Empty;

//nested struct
#[derive(Debug, PartialEq)]
struct Circle {
    radius: u32,
    center: Point
}

fn main() {
    let p = Point {x: 1 , y:1};
    println!("{:?}", p);
    println!("px: {}, py: {}", p.x , p.y);
    let p = Point3D(1,2,3);
    println!("p0: {}, p1: {}, p2: {}", p.0, p.1, p.2);

    let empty = Empty;
    println!("empty: {:?}", empty);

    let circle = Circle{
        radius: 5,
        center: Point {x: 0 , y:0}
    };

    println!("{:#?}", circle);

    //shortcut
    let x: i32 = 1;
    let y: i32 = 1;
    let p = Point{x,y};

    //copy fields
    let po = Point {x: 1 , y:1};
    let p1 = Point {x:2, ..po};
    println!("p1: {:?}", p1);

    //update
    let mut p2 = Point {x:3, y:3};
    p2.x = 4;
    println!("p2: {:?}", p2);

}