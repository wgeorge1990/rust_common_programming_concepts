fn longest(x: &str, y: &str) -> &str {
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
