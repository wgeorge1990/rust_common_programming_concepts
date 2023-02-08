use rusts_approach_to_inheritance::{Button, Screen, SelectBox};



fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width:75,
                height:10,
                options: vec![
                    String::from("yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    screen.run();
}
