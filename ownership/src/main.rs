fn main() {
    let mut s = String::from("hello");
    println!("s = {}", s);
    s.push_str(", world!");
    println!("s = {}", s);

    //copied and save to the stack in two differnt places
    let x = 5;
    let y = x;
    println!("{}", y);

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
    // in rust because pointer,length,capacity and initial variable invalidation occurs,
    // this is called a move oppose to a shallow copy.
    // RUST WILL NEVER CREATE DEEP COPIES OF YOUR DATA
    // which means automatic copying is considered inexpensive in terms
    // of runtime performance.

    // returning multiple value from a function using a tuple
    let string_one = String::from("hello");
    // string_one is moved and returned by calculate_length
    // and then moved to string_two via assignment and legth is calculated and passed out via second
    // argument of the tuple.
    let (string_two, len) = calculate_length(string_one);

    println!("The length of '{}' is {}", string_two, len);

    // References and borrowing
    // define and use function that has reference to an
    //    object as a parameter instead of taking ownership
    //    of the value.
    // & fn(s: &String) -> usize {} and fn(&str) 
    // &known as refernce: allows refering to some value
    // without having to take ownership of it.

    let rs1 = String::from("hello");
    let len2 = calculate_length_with_reference(&rs1);
    println!("The length of '{}' is {}.", rs1, len2);
}

    fn calculate_length_with_reference(s: &String) -> usize { // s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of
    //   what it refers to, nothing happens.
    // ownership is never passed and therefor never has to be given back.
    // This process of referencing parameters is called 'borrowing'





fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
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
