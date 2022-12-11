use std::fmt::Display;
fn main() {
    println!(" generic type parameters, trait bounds, and lifetimes together");
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
        where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else if x.len() == y.len() {
            "even"
        } else {
            y
        }
    }

    println!("{}",longest_with_an_announcement("xssd", "cac", "the largest will be printed below!"));
}

//Learn more advanced scenarios in the rust-lang documentation via the below url.
//https://doc.rust-lang.org/stable/reference/trait-bounds.html#lifetime-bounds.