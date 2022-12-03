fn main() {
    println!("Vector");
    // let v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    let v = vec![1,2,3,4,5];

    // accessing a vector
    // method one
    let third: &i32 = &v[2];
    let fourth = &v[3];
    println!("The third element is {}", third);
    println!("The third element is {}", fourth);

    // method two
    // using match with two arms: Some and None
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let does_not_exist = v.get(100);
    let does_exist = v.get(1);
    println!("{:?}",v);
    println!("{:?}", does_not_exist);
    println!("{:?}", does_exist);

    for i in &v {
        println!("{}", i);
    }

    // use the dereference operator(*) to get to the 
    // value in i before we can use the -= operator
    let mut v2 = vec![100,200,300];
    for i in &mut v2 {
        *i -= 99;
    }

    println!("{:?}", v2);

    //Storing enum variants in a vector to keep the 
    //type the same accross the vector
    

}
