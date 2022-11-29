fn main() {
    // let s = "hello";
    let mut s = String::from("hello");
    println!("s = {}", s);
    s.push_str(", world!");
    println!("s = {}", s);
}
// the stack and the heap
// both are parts of memory that are available
// to your code to use at runtime however they
// are structured in differnt ways...
// THE STACK:
//      store values in the order it gets them
//      and removes the order in opposite order
//  example: cooking and eating pancakes
//  term: last in, first out
//  use term(s): 
//     adding data:  'pushing onto the stack'
//     removing data: 'popping off of the stack'
// ALL STACK DATA MUST HAVE A KNOW FIXED SIZE
// otherwise at compile time it must be stored on
//    the heap.



