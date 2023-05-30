enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil))); DONT DO, instead do below... pg.322 Rust for Crustacians
    // In order for compiler to calculate List enum largest size use Box<List> like below:

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}
