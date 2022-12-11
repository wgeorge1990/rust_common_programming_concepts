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

#[cfg(test)]
mod tests {
    use super::*;

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
}
//assert! macro evaluates to a boolean, if true, assert! does nothing and test passes.
//Benchmark test documentation
//https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html
