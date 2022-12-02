use std::{thread::sleep, time::Duration};
const CLEAR: &str = "\x1B[2J\x1B[1;1H";
struct Progress<Aiter> {
    iter: Aiter,
    i: usize,
    bound: Option<usize>,
}
impl<Aiter> Progress<Aiter> {
    pub fn new(iter: Aiter) -> Self {
        Progress {
            iter,
            i: 0,
            bound: None,
        }
    }
}
impl<Aiter> Progress<Aiter>
where
    Aiter: ExactSizeIterator,
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}
impl<Aiter> Iterator for Progress<Aiter>
where
    Aiter: Iterator,
{
    type Item = Aiter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => println!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i)),
            None => println!("{}", "*".repeat(self.i)),
        }
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}
impl<Aiter> ProgressIteratorExt for Aiter {
    // where clause provent wrong iterator type
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}
fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}
fn main() {
    //let x = 1.progress();
    //let y = "blah".progress();
    //for n in (0 .. ).progress().with_bound() {
    //expensive_calculation(&n);
    //}
    let v = vec![1, 2, 3, 4];
    for n in v.iter().progress().with_bound() {
        expensive_calculation(n);
    }
}
