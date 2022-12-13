// https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759
// Closures are a combination of a function pointer (fn) and a context.
// A closure with no context is just a function pointer.
// A closure which has an immutable context belongs to Fn.
// A closure which has a mutable context belongs to FnMut.
// A closure that owns its context belongs to FnOnce.

struct MyStruct {
    text: &'static str,
    number: u32,
}
impl MyStruct {
    fn new (text: &'static str, number: u32) -> MyStruct {
        MyStruct { text, number }
    }
    fn get_number(&self) -> u32 {
        self.number
    }
    fn inc_number(&mut self) {
        self.number += 1;
    }
    fn destructor (self) {
        println!("Descructing  {}", self.text);
    }
}
fn is_fn<A, R>(_x: fn(A) -> R) {}
fn is_Fn<A, R, F: Fn(A) -> R>(_x: &F){}
fn is_FnMut<A, R, F: FnMut(A) -> R>(_x: &F) {}
fn is_FnOnce<A, R, F: FnOnce(A) -> R>(_x: &F) {}
fn main() {
    println!("starting program");
    // no context closure
    let obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("more text", 12);
    let closure1 = |x: &MyStruct| x.get_number() + 3;
    assert_eq!(closure1(&obj1), 18);
    assert_eq!(closure1(&obj2), 15);
    is_fn(closure1);
    is_Fn(&closure1);
    is_FnMut(&closure1);
    is_FnOnce(&closure1);
    // function 
    fn func1(x: &MyStruct) -> u32 {
        x.get_number() + 3
    }
    assert_eq!(func1(&obj1), 18);
    assert_eq!(func1(&obj2), 15);
}
#[test]
fn fn_immutable_context_test() {
    let obj1 = MyStruct::new("hello", 15);
    let obj2 = MyStruct::new("more text", 12);
    let closure = |x: &MyStruct| {
        x.get_number() + obj1.get_number()
    };
    assert_eq!(closure(&obj2), 27);
    //obj1.inc_number();
}

#[test]
fn fn_mutable_context_test() {
    let mut obj1 = MyStruct::new("hello", 15);
    let obj2 = MyStruct::new("more text", 12);
    let mut closure = |x: &MyStruct| {
        obj1.inc_number();
        x.get_number() + obj1.get_number()
    };
    assert_eq!(closure(&obj2), 28);
    assert_eq!(closure(&obj2), 29);
    assert_eq!(closure(&obj2), 30);
    obj1.inc_number();
    assert_eq!(obj1.get_number(), 19);
}
#[test]
fn fn_mutable_context_func_test() {
    struct Context<'a>(&'a mut MyStruct);
    let mut obj1 = MyStruct::new("hello", 15);
    let obj2 = MyStruct::new("more text", 12);
    let mut ctx = Context(&mut obj1);
    fn func(context: &mut Context, x:&MyStruct)->u32 {
        context.0.inc_number();
        x.get_number() + context.0.get_number()
    }
    assert_eq!(func(&mut ctx, &obj2), 28);
    assert_eq!(obj1.get_number(), 16);
}
#[test]
fn test_fnonce_owned_context () {
    let obj1 = MyStruct::new("hello", 15);
    let obj2 = MyStruct::new("more text", 12);
    let closure = |x: &MyStruct| {
        obj1.destructor();
        x.get_number()
    };
    assert_eq!(closure(&obj2), 12);
    //obj1.inc_number();  // error
}
