fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    println!("Generics");
    //Removing duplication by extracting a function
    let number_list = vec![34, 50, 25, 100, 65];
    let high_number = largest(&number_list);
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("The largest number is {}", high_number);

    //find the largest number in the next vector
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let high_number = largest(&number_list);
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("The largest number in the next vector is {}", high_number);
}
