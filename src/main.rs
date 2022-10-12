struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn display(&self) {
        println!("Value:\n_ \n_*|{}", self.value);
        if let Some(left) = &self.left {
            println!("LEFT");
            left.display();
        }

        if let Some(right) = &self.right {
            println!("RIGHT");
            right.display();
        }
        println!("--------UP--------")
    }
}
fn main() {
    let mut root = Node {
        value: 2,
        left: Some(Box::new(Node {
            value: 7,
            left: Some(Box::new(Node {
                value: 7,
                left: None,
                right: None,
            })),
            right: None,
        })),
        right: Some(Box::new(Node {
            value: 4,
            left: None,
            right: None,
        })),
    };
    root.display();
}
