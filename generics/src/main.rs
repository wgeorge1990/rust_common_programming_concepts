fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

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
    println!("Generics");
    //Removing duplication by extracting a function
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    // code block replaced by initial recactor
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    //find the largest number in the next vector
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number in the next vector is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
