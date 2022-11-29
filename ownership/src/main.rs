fn main() {
    let mut s = String::from("hello");
    println!("s = {}", s);
    s.push_str(", world!");
    println!("s = {}", s);

    //copied and save to the stack in two differnt places
    let x = 5;
    let y = x;

    //consists of three parts stored on the stack:
    // a pointer to the memory that hold the contents of the string
    // a length(how much memory in bytes)
    // a pointer
    // the capacity is the total amount of memory in bytes that the string
    // has recieved from the operating system.

    let s1 = String::from("hello");
    let s2 = s1;
    // s1 and s2 both exist on the heap 
    //  as a pointer to the index and value on the stack,
    //  length,
    //  and capacity.
    // println!("{}, world!", s1); rust prevents user from using
    //invalidated reference which causes a double free error which 
    //occurs when the same memory is tried to be cleared twice.
    println!("{}, world!", s2);

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



