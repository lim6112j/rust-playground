use std::{thread, time::Duration};

// https://doc.rust-lang.org/book/ch13-01-closures.html?highlight=closure#capturing-the-environment-with-closures
//
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    //println!("string to example closure : {}", n);

    let list: Vec<i32> = vec![1, 2, 3];
    let only_borrow = || println!("From closure: {:?}", list);
    only_borrow();
    let mut list2 = vec![4, 5, 6];
    let mut borrow_mutably = || list2.push(7);
    borrow_mutably();
    thread::spawn(move || {
        println!(
            "From thread - closure take ownership of variable: {:?}",
            list
        )
    })
    .join()
    .unwrap();
}
