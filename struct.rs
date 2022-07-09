#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
struct Unit;
struct Pair(i32, f32);
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
fn main() {
    let name = String::from("Peter");
    let age = 28;
    let peter = Person { name, age };
    println!("{:?}", peter);
    let point: Point = Point { x: 10.0, y: 10.0 };
    println!("Point coordinates x: {}, y: {}", point.x, point.y);
    let bottom_right = Point { x: 5.2, ..point };
    println!(
        "bottom_right coordinates x: {}, y: {}",
        bottom_right.x, bottom_right.y
    );
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair of first {}, second {}", pair.0, pair.1);
    let Pair(_integer, _decimal) = pair;
    println!("pair : ({}, {})", _integer, _decimal);
}
