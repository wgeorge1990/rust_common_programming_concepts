// the idiomatic way to bring the standard libs HashMap
//   struct into the scope of a binary crate; 
use std::collections::HashMap;
// pub allows external code to refer to hosting module as well
// this is called re-exporting.
// moving front of house to front_of_house.rs and add bellow line.
mod front_of_house;
pub use crate::front_of_house::hosting;
mod back_of_house {
    //public scoping works the same in modules as it
    //does in structs. ENUMs on the other hand work as a whole.
    // structs are private by default, unless told otherwise.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    //enums variants are public by default.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

}

// public functions in crate or root scope
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Order breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Then change our mind about the bread via
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
    // above line will not work if uncommented

    // Absolute path
    hosting::add_to_waitlist();
    
    // Relative path
    hosting::add_to_waitlist();
    // with use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
