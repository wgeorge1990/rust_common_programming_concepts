fn main() {
// x times loop using loop with break statement 
//   and then returning said value of x
    let mut counter = 0;

    let result = loop {
            counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

// while loop with subtraction
    let mut another_number = 3;

    while another_number != 0 {
        println!("{}!", another_number);
    another_number -= 1;
    }

println!("LIFTOFF!!!");

// Using while loop to interate an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value in: {}", a[index]);
        index += 1;
    }
}
