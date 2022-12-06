fn main() {
    //vectors_and_such();
    // string_collections();
    // iterating_over_strings();
    storing_keys_and_values_in_hasmaps();
}

fn vectors_and_such() {
    println!("Vector");
    // let v: Vec<i32> = Vec::new();
    // v.push(1);

    let v = vec![1,2,3,4,5];

    // accessing a vector: method one
    let third: &i32 = &v[2];
    let fourth = &v[3];
    println!("The third element is {}", third);
    println!("The third element is {}", fourth);

    // method two: using match with two arms: Some and None
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
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}

fn string_collections() {
    println!("string collections:");
    //Strings are UTF-8 encoded
    let s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    println!("{}", s);
    let mut s2 = String::from("foo");
    s2.push_str(" bar");
    println!("{}", s2);
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);
    println!("{:?}",tic_tac_toe);

    let hello = "Здравствуйте";
    println!("{}", hello);
    let h = &hello[0..2];
    // line 80 causes program to panic and quit because
    // the the complete char is stored in 2 byte peices 
    // oppose to one like some characters. This is why
    // working with strings in trickier than some think.
    // let e = &hello[0..1];
    println!("{}", h);
    // println!("{}", e);

}

fn iterating_over_strings() {
    for c in "hello world".chars() {
        println!("{}", c);
    }

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    // remember that valid Unicode scalar values may be 
    // made up of more than 1 byte.

    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }

    for b in "hello world".bytes() {
        println!("{}", b);
    }

}

fn storing_keys_and_values_in_hasmaps() {
    println!("storing_keys_and_values_in_hasmaps()");
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.insert(String::from("Yellow"), 50);

    println!("{:?}",scores);

    // for (key, value) in scores.iter() {
    //     println!("{}:{}", key, value);
    // }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("{} {}", team_name, score),
        None => println!("key not found"),
    };

    let team_vec = vec![
        String::from("One"), 
        String::from("Two"), 
        String::from("Three"), 
        String::from("Four"), 
        String::from("Five") ];

    let score_vec = vec![
        10,
        20,
        30,
        40,
        50 ];

    let team_scores: HashMap<_, _> = team_vec.iter().zip(score_vec.iter()).collect();

    // notice how the final hash map consists of the paralell team name and score value
    // but when printed, notice that the keys are ordered a..z by default.
    println!("{:?}", team_scores);




}