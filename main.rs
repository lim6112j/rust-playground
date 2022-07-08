use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
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
    let mut statesCode = HashMap::new();
    statesCode.insert("KL", "Kerala");
    statesCode.insert("MH", "Mahashtra");
    println!("size of map is {:?}", statesCode);

    match statesCode.get(&"KL") {
        Some(value) => {
            println!("Value of key KL is {}", value);
        }
        None => {
            println!("nothing found");
        }
    }
    for (key, val) in statesCode.iter() {
        println!("key: {} , val: {}", key, val);
    }
    if statesCode.contains_key(&"KL") {
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
}
