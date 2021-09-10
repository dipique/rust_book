use oop_features::{Screen, Button};
use selectbox::SelectBox;

pub mod selectbox;

pub fn draw_screen() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
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
