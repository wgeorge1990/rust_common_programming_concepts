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
   
    #[derive(Debug)]
    struct ImportantExcerp<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me. Call me again years ago...");
        let first_sentence = novel
            .split('.')
            .next()
            .expect("No period or next segment");

        println!("{:?}", first_sentence);
    let imp_exerp = ImportantExcerp {
        part: first_sentence
    };
    println!("{:?}", imp_exerp);
    

}
