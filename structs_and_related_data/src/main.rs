fn main() {
    println!("Structs and structuring related data!");
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        username: String::from("willg529"),
        email: String::from("someexample@example.com"),
        active: true,
        sign_in_count: 1,
    };

    // refactor to use feild init shorthand becasue the fields
    // of the user struct and the parameter names are exactly! the same.
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    };

    user1.email = String::from("example@example.com");

    let mut user2 = build_user(String::from("wg@ex.com"), String::from("wg"));

    println!("username: {}", user2.username);
    println!("email : {}", user2.email);
    println!("active: {}", user2.active);
    println!("sign_in_count: {}", user2.sign_in_count);
}


