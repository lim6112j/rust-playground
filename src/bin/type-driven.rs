use std::{thread::sleep, time::Duration};
const CLEAR: &str = "\x1B[2J\x1B[1;1H";
struct Progress<Aiter> {
    iter: Aiter,
    i: usize
}
impl<Aiter> Progress< Aiter> {
    pub fn new(iter: Aiter) -> Self {
        Progress {iter, i: 0}
    }
}
impl<Aiter> Iterator for Progress<Aiter> where Aiter: Iterator{
    type Item = Aiter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}
impl<Aiter> ProgressIteratorExt for Aiter {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}
fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}
fn main() {
    let v = vec![1, 2, 3];
    for n in v.iter().progress() {
        expensive_calculation(n);
    }
}
