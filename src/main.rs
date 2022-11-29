use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

mod my;
// enum with types
#[derive(Debug)]
enum GenderCategory {
    Name(String),
    UsrId(i32),
}
// Result enum
fn is_even(no: i32) -> Result<bool, String> {
    if no % 2 == 0 {
        return Ok(true);
    } else {
        return Err("Not an even".to_string());
    }
}
// Generics
struct Data<T> {
    value: T,
}
struct Book {
    name: &'static str,
    id: u32,
}
trait Printable {
    fn print(&self);
}
impl Printable for Book {
    fn print(&self) {
        println!("Printing book with id:{}, name: {}", self.id, self.name);
    }
}

fn main() {
    // from struct.rs

    let name = String::from("Peter");
    let age = 28;
    let peter = my::my::Person { name, age };
    println!("{:?}", peter);
    let point: my::my::Point = my::my::Point { x: 10.0, y: 10.0 };
    println!("Point coordinates x: {}, y: {}", point.x, point.y);
    let bottom_right = my::my::Point { x: 5.2, ..point };
    println!(
        "bottom_right coordinates x: {}, y: {}",
        bottom_right.x, bottom_right.y
    );
    let my::my::Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let _rectangle = my::my::Rectangle {
        top_left: my::my::Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };
    let _unit = my::my::Unit;
    let pair = my::my::Pair(1, 0.1);
    println!("pair of first {}, second {}", pair.0, pair.1);
    let my::my::Pair(_integer, _decimal) = pair;
    println!("pair : ({}, {})", _integer, _decimal);
    // end of struct.rs
    let p1 = GenderCategory::Name(String::from("lim"));
    let p2 = GenderCategory::UsrId(10);
    match p1 {
        GenderCategory::Name(val) => {
            println!("{}", val);
        }
        GenderCategory::UsrId(val) => {
            println!("{}", val);
        }
    };
    // vector is resizable array !
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    // borrowing v
    for i in &v {
        println!("vec v contains : {}", i);
    }
    println!("{:?}", v); // v is alive for borrowing

    // map is a collection of key-value pairs (aka entries)
    // Hashmap stores keys and values in hash table.
    let mut states_code = HashMap::new();
    states_code.insert("KL", "Kerala");
    states_code.insert("MH", "Mahashtra");
    println!("size of map is {:?}", states_code);

    match states_code.get(&"KL") {
        Some(value) => {
            println!("Value of key KL is {}", value);
        }
        None => {
            println!("nothing found");
        }
    }
    for (key, val) in states_code.iter() {
        println!("key: {} , val: {}", key, val);
    }
    if states_code.contains_key(&"KL") {
        println!("statesCode contains KL");
    }
    let mut names = HashSet::new();
    names.insert("lim");
    names.insert("kutan");
    if names.contains(&"lim") {
        println!("names contains lim");
    }
    /*
    Unrecoverable error : panic!()
    Recoverable error : Result enum
    */
    let f = File::open("main.jpg");
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        }
        Err(e) => {
            println!("file not found {}", e);
        }
    }
    println!("5 is even ? : {:?}", is_even(5));
    // unwrap, expect is like join in haskell , extract value of Option, Result enums or panic
    let is_even_value = is_even(10).unwrap();
    println!("10 is even : {:?} ", is_even_value);
    let is_even_value = is_even(10).expect("not even panic");
    println!("10 is even : {:?}", is_even_value);

    // Generics
    let t: Data<String> = Data {
        value: "hello".to_string(),
    };
    println!("data value is {}", t.value);
    // traits are like interfaces of oop
    let book = Book {
        id: 1,
        name: "wind of change",
    };
    book.print();
    // smart pointers Box, Deref, Drop
    let var_i32: i32 = 10;
    let b = Box::new(var_i32);
    println!("smart pointer value : {}", *b == var_i32);
    // primitives from lust-lang example
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    let mut mutable = 12;
    mutable = 21;
}
