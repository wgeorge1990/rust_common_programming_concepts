//examples of lifetime annotations
// &i32 a reference
// &'a i32 a reference with an explicit lifetime
// &'a mut i32 a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else if y.len() > x.len() {
        y
    } else {
        "even"
    }
}

fn main() {
    println!("validating references with lifetimes!");

    let string1 = String::from("abcdefg");
    let string2 = "abcd";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
