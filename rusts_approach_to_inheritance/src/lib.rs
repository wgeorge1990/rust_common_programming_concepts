pub trait Draw {
     fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

//define the method run, on the Screen struct, that calls
//the draw method on each of its components.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //code to actually draw button
        println!("{:?}", self);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //code to draw a select box
        // println!("{:?}", self);
    }
}