fn main() {
    println!("functions:");
    another_function();
    function_with_params(100);
    print_five();
}


fn another_function() {
    println!("Another Function");
}


fn function_with_params(x: i32) {
    println!("function with params:");
    println!("The value of x is: {}", x);
}


// Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, 
//   you turn it into a statement, which will then not return a value.

fn five() -> i32 {
    5
}
 fn print_five() {
    let x = five();
    println!("The value of x is: {}", x);
}

