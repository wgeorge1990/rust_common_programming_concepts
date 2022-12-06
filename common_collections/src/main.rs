fn main() {
    //vectors_and_such();
    // string_collections();
    // iterating_over_strings();
    // storing_keys_and_values_in_hasmaps();
    // overwriting_a_value_in_a_hashmap();
    // inserting_value_for_key_with_no_value();
    updating_value_based_on_old_value();
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

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

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

fn overwriting_a_value_in_a_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 111);
    scores.insert(String::from("Blue"),222);

    println!("{:?}", scores);
}

fn inserting_value_for_key_with_no_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // using entry to check for key value.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // =>   {"Blue": 10, "Yellow": 50}
    // " The or_insert method on Entry is defined to return a mutable reference to the value 
    //    for the corresponding Entry key if that key exists, and if not, inserts the 
    //    parameter as the new value for this key and returns a mutable reference to the new value."
    //  TODO: (add source info and formatting) 
    //         The Rust Programming Language (Covers Rust 2018) - Kindle Edition.
}

fn updating_value_based_on_old_value() {
    use std::collections::HashMap;
    
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        //count stores the mutable reference to the value,
        //so in order to modify it we need to dereference it first
        // with the * which holds strong to the borrowing rules
        // in place by the language rules.
        *count += 1;
    }
    println!("{:?}", map);
    // => {"wonderful": 1, "world": 2, "hello": 1}
    // 1}. The or_insert method actually returns a mutable reference (&mut V) to the value for this key. 
    // Klabnik, Steve; Nichols, Carol. The Rust Programming Language (Covers Rust 2018) (p. 148). No Starch Press. Kindle Edition. 

}

