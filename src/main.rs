mod string_utils;
mod gui;
mod blog;
mod test_trait;

use string_utils::to_hex_array::to_hex_array_fn;

use std::env;

use std::ops::Deref;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        let arg1 = &args[1];
        let arg2 = &args[2];

        if arg1.eq("tohex") {
            to_hex_array_fn(arg2);
        }
    }
}
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data");
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
#[test]
fn test_box() {
    let box1 = MyBox::new(5);
    println!("box = {:?}", box1);
}
