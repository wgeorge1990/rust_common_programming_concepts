fn main() {
    println!("Structs and structuring related data!");
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

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

    let mut user1 = User {
        username: String::from("willg529"),
        email: String::from("someexample@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("example@example.com");

    let mut user2 = build_user(String::from("wg@ex.com"), String::from("wg"));

    //struct update syntax
    let user3 = User {
        email: user2.email,
        username: user2.username,
        active: false,
        sign_in_count: 3,
    };

    //struct update syntax with spread operator
    let user4 = User {
        active: false,
        sign_in_count: 3,
        ..user3
    };

    println!("username: {}", user4.username);
    println!("email : {}", user4.email);
    println!("active: {}", user4.active);
    println!("sign_in_count: {}", user4.sign_in_count);

    //tuple structs
    
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    // black and origin values are different types, 
    // because they're instances of differnt tuple structs.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
    
    println!("{}", origin.0);
    println!("{}", origin.1);
    println!("{}", origin.2);
}


