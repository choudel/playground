use model::Hands;

mod model {
    pub trait Displayable{
        fn display(&self)->String;
    }
    enum Fruit {
        Apple,
        Banana,
        Kiwi,
    }
    impl Displayable for Fruit {
        fn display(&self) -> String {
            match self {
                Fruit::Apple => "an apple".to_owned(),
                Fruit::Banana => "a banana".to_owned(),
                Fruit::Kiwi => "a kiwi".to_owned(),
            }
        }
    }
    pub struct Hands {
        left: Option<Fruit>,
        right: Option<Fruit>,
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
            report_item(&self.left, "left");
            report_item(&self.right, "right");
        }
        pub fn new() -> Self {
            Self {
                left: Some(Fruit::Apple),
                right: Some(Fruit::Banana),
            }
        }
    }
   
    
        pub fn report_item<T:Displayable>(item:&Option<T>, which: &str) {
            match item {
                Some(what) => {
                    println!("{} hand is holding {}", which, what.display())
                }
                _ => {
                    println!("{} hand is holding nothing", which)
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
