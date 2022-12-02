struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("calculate area of rectangle");

    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30, 50);

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
