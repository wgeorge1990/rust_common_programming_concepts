//A summary trait that consists of the behavior provided
//  by the summarize method.
// to use a trait from a library one would add:
// use aggregator::Summary to gain access to the given trait
// also the summary trait would also need to be public for another
// crate to implement it.
// one restriction to note with trait implementation is that we
// can only implement a trait on a type only if either the trait
// or type is local to our crate.
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
} 

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", 
//         self.headline, 
//         self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


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
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people are"),
        reply: false,
        retweet: false,
    };

    // test sumarize trait method implemented on the tweet instance
    println!("1 new tweet: {}", tweet.summarize());

    let news_article = NewsArticle {
        headline: String::from("NewsArticle headline"),
        location: String::from("Atlanta"),
        author: String::from("William George"),
        content: String::from("This is content but the default trait 
        implementation for summary should run becaue the summarize block implementation is empty for NewsArticle type.")
    };

    //default trait summary should be printed from below function because
    //the trait implemtation for summarize is an empty block.
    println!("1 new newsarticle: default behavior expected: {}", news_article.summarize());

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
