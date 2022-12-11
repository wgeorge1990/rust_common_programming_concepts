//assert! macro evaluates to a boolean, if true, assert! does nothing and test passes.
//Benchmark test documentation
//https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html
//Other programming languages: actual and expected
//rust: left and right
//Threads: by default rust takes advantage of threads when running tests.
//If you want to use shared state or enviornment variables then ensure you run
//the tests on a single thread via $cargo test -- --test-threads=1
//To display output for passing tests run $cargo test -- --show-output
//Ignore tests by adding #[ignore] annotation
//run tests that include text by cargo test text
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn guess_acceptable() {
        Guess::new(99);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn adds_two_to_parameter() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("William");
        assert!(
            result.contains("William"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    //returning specific error using Result type
    //This enables you to use the question mark operator in the
    //body of tests, which can be a convient way to write tests that
    //should fail if any operation within them returns a Err variant.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
