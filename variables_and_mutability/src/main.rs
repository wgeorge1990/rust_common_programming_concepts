use std::io;
fn main() {
    let x = 5;
//  x is showdowing the previos value of x
 let x = x + 1;
//  x  = 6 becasue it is shadowing the previous value of x as 5 + 1 = 6
 let x = x + 2;
 println!("The value of x is: {}", x);

 let remainder = 43 % 5;
 println!("This is the remainder: {}", remainder);

 let tup = (500, 6.4, 1);
//  let (a, b ,c ) = tup;
 println!("tup index 0: {}", tup.0);
 println!("tup index 1: {}", tup.1);
 println!("tup index 2: {}", tup.2);
 let (fivehundred, six_point_four, one) = tup;
 println!("{}", fivehundred);
 println!("{}", six_point_four);
 println!("{}", one);

 array_access()
}

fn array_access() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line!");
    let index: usize = index.trim().parse().expect("Not a number");
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
