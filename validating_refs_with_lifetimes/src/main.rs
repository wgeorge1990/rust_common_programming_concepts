//examples of lifetime annotations
// &i32 a reference
// &'a i32 a reference with an explicit lifetime
// &'a mut i32 a mutable reference with an explicit lifetime

// THREE LIFETIME PARAMETER RULES
// 1.Each parameter that is a reference get its own lifetime parameter
// 2.If there is exactly one input lifetime parameter, then
//   that lifetime parameter is assigned to all output lifetime parameters.
// 3.If there are multiple input lifetime parameters, but one
//   of them is &self or &mut self becasue this is a method,
//   the lifetime of self is assigned to all output lifetime parameters.
//   This third rule makes methods much nicer to read and write because
//   fewer symbols are necessary.

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

    //The lifetime parameter declaration after impl 
    //and use after the type name is required, 
    //but we’re not required to annotate the lifetime 
    //of the reference to self because of the first elision rule.
    impl<'a> ImportantExcerp<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me. Call me again years ago...");
    let first_sentence = novel.split('.').next().expect("No period or next segment");

    println!("{:?}", first_sentence);
    let imp_exerp = ImportantExcerp {
        part: first_sentence,
    };
    println!("{:?}", imp_exerp);
    println!("{}",imp_exerp.announce_and_return_part("Announcement"));
}
