fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let y = if_as_statement(0);
    let i = if_as_statement(1);
    let z = if_as_statement(-1);
    // let v = if_as_statement("g");

    println!("y evalutates to: {}", y);
    println!("y evalutates to: {}", i);
    println!("y evalutates to: {}", z);
    // println!("y evalutates to: {}", v);
}

fn if_as_statement(x: i32) -> i32 {
    if x > 0 {
        1
    } else if x == 0 {
        0
    } else if x < 0 {
        -1
    } else {
        -2
    }
}
