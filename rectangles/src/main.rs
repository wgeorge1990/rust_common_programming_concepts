fn main() {
    println!("calculate area of rectangle");

    // let width1 = 30;
    // let height1 = 50;

    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        area(rect1)
    );
}

fn area( deminsions: (u32, u32) ) -> u32 {
    deminsions.0 * deminsions.1
}
