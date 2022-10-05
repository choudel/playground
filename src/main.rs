use model::Hands;

mod model {
    pub struct Item {
        what: String,
        present: bool,
    }
    impl Item {
        pub fn report_item(&self, which: &str) {
            if self.present {
                println!("{} hand is holding {}", which, self.what);
            } else {
                println!("{} hand is holding nothing", which);
            }
        }
    }
    pub struct Hands {
        left: Item,
        right: Item,
    }
    impl Hands {
        pub fn juggle(mut self) -> Self {
            println!("let's juggle");
            let air = self.left;
            self.left = self.right;
            self.right = air;
            self
        }
        pub fn report(&self) {
            Item::report_item(&self.left, "left");
            Item::report_item(&self.right, "right");
        }
        pub fn new() -> Self {
            Self {
                left: Item {
                    what: "an apple".to_owned(),
                    present: true,
                },
                right: Item {
                    what: "a banana".to_owned(),
                    present: true,
                },
            }
        }
    }
}

fn main() {
    let mut hands = Hands::new();

    hands.report();

    hands = hands.juggle();

    hands.report();
}
