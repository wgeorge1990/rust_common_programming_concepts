#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("calculate area of rectangle");

    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30, 50);

    let rect1 = Rectangle { 
        width: 30, 
        height: 50 
    };

    println!("rect1 is {:#?}", rect1);

    println!("The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        rect1.area()
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
