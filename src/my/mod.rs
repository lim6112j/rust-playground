pub mod my {
    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }
    pub struct Unit;
    pub struct Pair(pub i32, pub f32);
    #[derive(Debug)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }
    pub struct Rectangle {
        pub top_left: Point,
        pub bottom_right: Point,
    }
}
