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

//A use case for Interior Mutability: Mock Objects
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
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
use std::cell::RefCell;
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
       self.sent_messages.borrow_mut().push(String::from(message));
    //    let mut two_borrow = self.sent_messages.borrow_mut();
        // two_borrow.push(String::from(message));
    }
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    //create mock messenger instance for test
    let mock_messenger = MockMessenger::new();
    //create limit tracker from crate::LimitTracker above
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    //use setValue method to aggree with the evaluation in the test name
    limit_tracker.set_value(80);

    //assert equal that the message is present and the length is eqaul to one
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}
