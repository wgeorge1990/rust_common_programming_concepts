fn main() {
    println!("Hello, world!");
    let s: String = String::from("hello world");

    // let length: usize = first_word(&s);
    let length: &str = first_word(&s);

    println!("string length: {}", s.len());
    println!("The length of the first word is: {}", length);
    println!("{}", length);
}


// Here’s a small programming problem: write a function 
// that takes a string and returns the first word it 
// finds in that string. If the function doesn’t find a 
// space in the string, the whole string must be one word, 
// so the entire string should be returned.

// fn first_word(s: &String) -> usize {
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // searching for space in string using byte literal syntax: b' '
        if item == b' ' {
            println!("item: '{}' at index {}", item, i);
            // return &s[0..i];
                //or
            return &s[..i];
        }
    }
    &s[..]
}
