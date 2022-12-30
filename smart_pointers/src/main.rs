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

fn main() {
    println!("Smart Pointers");

}
