fn main() {
    println!("Hello, world!");
    let s: String = String::from("hello world");
    let s2: String = String::from("thisisonewholeword");
    // let length: usize = first_word(&s);
    let word = first_word(&s);
        // or
    let word2 = first_word(&s2);

    println!("string length: {}", s.len());
    println!("The length of the first word is: {}", word);
    println!("{}", word);
    println!("{}", word2);

}


// Here’s a small programming problem: write a function 
// that takes a string and returns the first word it 
// finds in that string. If the function doesn’t find a 
// space in the string, the whole string must be one word, 
// so the entire string should be returned.

// fn first_word(s: &String) -> usize {
fn first_word(s: &str) -> &str {
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
