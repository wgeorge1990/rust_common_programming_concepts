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

// function that returns a value
//   becasue line 26 outputs a value it is considered a statement and there for
//   requires no semicolon after the 5 and also requires the return type to be
//   identified ahead of time using -> _type_ inbetween the params holder and the fn block {}
fn five() -> i32 {
    5
}


 fn print_five() {
    let x = five();
    println!("The value of x is: {}", x);
}

