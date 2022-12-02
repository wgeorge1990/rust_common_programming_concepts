#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // use mut &self if we want to makes changes to the Rectangle fields.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //namespaced with :: just like with modules
    //associated functions: call with Rectangle::square(size: u32)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    println!("calculate area of rectangle");
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);

    let square1 = Rectangle::square(5);

    let rect1 = Rectangle { 
        width: 30, 
        height: 50 
    };

    let rect2 = Rectangle { 
        width: 10, 
        height: 40 
    };

    let rect3 = Rectangle { 
        width: 60, 
        height: 45 
    };

    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:#?}", rect2);
    println!("rect1 is {:#?}", rect3);
    println!("rect1 is {:#?}", square1);

    println!("The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        rect1.area()
    );

    println!("The area of the Rectangle::square is {} square pixels.",
        // area(width1, height1)
        square1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
