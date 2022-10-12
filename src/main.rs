use std::{rc::Rc, vec};

struct Node {
    value: &'static str,
    edges:Vec<Rc<Node>>
}
impl Node {
    fn display(&self) {
        println!("Value:\n_ \n_*|{}", self.value);
        for edge in &self.edges{
          edge.display();
        }
        println!("--------UP--------")
    }
}
fn main() {
    let e = Rc::new(Node{
      value:"e",
      edges: vec![]
    });
    let d = Rc::new(Node{
      value:"d",
      edges: vec![e.clone()]
    }) ;
    let a =Node{
      value:"a",
      edges:vec![
        Rc::new(Node{
          value:"b",
          edges:vec![d.clone()],
        }),
        Rc::new(Node{
          value:"c",
          edges:vec![d.clone(),e.clone()],
        }),
        d,
        e
      ]
    };
    a.display();
}
