trait Animal {
    fn eat(&self) {
        println!("animal eat");
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn eat(&self) {
        println!("animal eat {:?}", &self.name);
    }
}

#[test]
fn test() {
    let list = vec![
        Dog {
            name: String::from("dog1"),
        },
        Dog {
            name: String::from("dog2"),
        },
    ];

    for dog in list {
        dog.eat();
    }
}
