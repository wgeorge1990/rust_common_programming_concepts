fn main() {
    println!("Object Oriented Programming in Rust!");
    println!("objects contain data and behavior...YES");
    println!("Encapsulation of data..YES");

    println!("Inheritance...NO...HOWEVER");
    println!(
        "In RUST we can use default trait method implemeentations
    instead of inheritance architechture normally found in OOP."
    );

    println!(
        "The other reason relates to type system and enables a child 
    type to be used in the same place as a parent type. This is also called
    polymorphism, which means you can substitute multiple objects for each other at runtime
    if they share certain characteristics."
    );

    let mut avg_collection = AveragedCollection {
        list: vec![],
        average: 0.0,
    };
    avg_collection.add(10);
    avg_collection.add(11);
    avg_collection.add(12);
    avg_collection.add(40);
    avg_collection.add(32);
    avg_collection.remove();

    println!("{:?}", avg_collection.average());
}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
