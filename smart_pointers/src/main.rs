//! # SMART POINTERS
//! - Smart pointers are usually implemented using structs.
//! The characteritic that distinguishes a smart pointer from an
//! ordinary struct is that smart pointers implement Deref and Drop traits.
//! Deref - allows it to behave like a ref so you can use them to work with both.
//! Drop - allow you to customize the code that is run when an instance
//! of the smart pointer goes out of scope.
//! ## MOST COMMON SMART POINTER IN STD LIB.
//! - Box<T> for allocating values on the heap
//! - Rc<T>, a reference counting type that enables multiple ownership
//! - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
//! - Review chapter 4 for difference between the stack and the heap!
//!
use crate::List::{Cons, Nil};

use std::rc::Rc;

use crate::SharedList::{Cons as OtherCons, Nil as OtherNil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use std::ops::Deref;
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        //rust calls this automatically when the individual instances
        //go out of scope.
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

//Using Rc<T> to Share Data
//see enums List for example below...
enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

fn main() {
    println!("Use a box to store an i32 value on the heap");
    let b = Box::new(5);
    println!("b = {}", b);

    println!("use box to represent recursive programming");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = &x;
    println!("{}", *y);

    let z = 100;
    let i = MyBox::new(z);
    println!("{}", *i);

    // customSmartPointer that implements the drop trait and demonstrating where we
    //   would place our cleanup code in realation to the instances of the smartpointers.
    let c = CustomSmartPointer {
        data: String::from("cleanup for var c"),
    };
    let d = CustomSmartPointer {
        data: String::from("Cleanup for var d"),
    };
    println!("CustomSmartPointer created.");

    //using Rc<T> to share a and b's ownership of a third list, a
    //using Box<T> will result in a use of moved value: 'a' error
    let aa = Rc::new(OtherCons(5, Rc::new(OtherCons(10, Rc::new(OtherNil)))));
    println!("Count after creating aa = {}", Rc::strong_count(&aa));
    let bb = Rc::new(OtherCons(3, Rc::clone(&aa)));
    println!("Count after creating bb = {}", Rc::strong_count(&aa));
    {
        let cb = Rc::new(OtherCons(4, Rc::clone(&aa)));
        println!("Count after creating cb = {}", Rc::strong_count(&aa));
    }
    println!(
        "count after cb goes out of scope = {}",
        Rc::strong_count(&aa)
    );
}

// How Deref coercion interacts with mutability
// - from &T to &U when T: Deref<Target=U>
// - from &mut T to U when T: DerefMut<Target=U>
// - from 7mut T to &U when T: Deref<Target=U>
