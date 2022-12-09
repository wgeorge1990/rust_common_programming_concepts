// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//---REFACTORING STEPS-----:
//   1. Identify duplicate code.
//   2. Extract the duplicate code into the body of the function
//       and specify the inputs and return values of that code in the function signature.
//   3. Update the two instances of duplicated code to call the function instead.
//source => Klabnik, Steve; Nichols, Carol. The Rust Programming Language (Covers Rust 2018) (p. 174). No Starch Press. Kindle Edition.

fn main() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    // personal example for learning's sake
    #[derive(Debug)]
    struct Name<F, L> {
        first: F,
        last: L,
    }

    impl<F, L> Name<F, L> {
        fn swap_last_names<A, B>(self, other: Name<A, B>) -> Name<F, B> {
            Name {
                first: self.first,
                last: other.last,
            }
        }
    }

    let n1 = Name {
        first: "will",
        last: "george",
    };
    let n2 = Name {
        first: "mackie",
        last: "marcello",
    };
    let n3 = n1.swap_last_names(n2);
    println!("{:?}", n3);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let will_work = Point { x: 5, y: 4 };
    // wont work yet because T type is the same for x and
    // y in the above struct. need to fix...
    let wont_work = Point { x: 5, y: 4.0 };
    println!("{:?}", will_work);
    println!("{:?}", wont_work);
    println!("Generics");
    //Removing duplication by extracting a function
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // ---- code block replaced by initial recactor ----
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    //find the largest number in the next vector
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest(&number_list);
    // println!("The largest number in the next vector is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
