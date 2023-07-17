pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {
//         println!("draw Button");
//     }
// }
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw SelectBox");
    }
}

// #[test]
// fn test() {
//     let scr = Screen {
//         components: vec![Box::new(SelectBox {
//             width: 75,
//             height: 10,
//             options: vec![
//                 String::from("Yes"),
//                 String::from("Maybe"),
//                 String::from("No"),
//             ],
//         })],
//     };

//     scr.run();
// }
