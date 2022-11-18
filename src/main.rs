enum Color {
    Brown,
    Red,
}
impl Color {
    fn print_cl(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions {
    fn print_d(&self) {
        println!("width: {:?}", self.width);
    }
}
struct Box {
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}
impl Box {
    fn create_new(dimensions: Dimensions, weight: i32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print_b(&self) {
        self.color.print_cl();
        self.dimensions.print_d();
        println!("weight: {:?}", self.weight);
    }
}
fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = Box::create_new(small_dimensions, 5, Color::Red);
    small_box.print_b();
}
