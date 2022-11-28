
trait Clicky{
    fn click(&self);
}
#[derive(Debug)]

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) {
        println!("click clack")
    }
}
fn borrow_clicky(obj:&dyn Clicky){
    obj.click();
}
fn move_clicky(obj:Box<dyn Clicky>){
    obj.click();
}
#[derive(Debug)]

struct Mouse;
impl Clicky for Mouse{
    fn click(&self) {
        println!("click");
    }
}
fn make_clicks(clickeys:Vec<Box<dyn Clicky>>){
    for click in clickeys{
     click.click();
    }
}
fn main() {
let keeb = Box::new(Keyboard);
let mouse =Box::new(Mouse);
let clickers: Vec<Box<dyn Clicky>> = vec![keeb,mouse];
make_clicks(clickers);
}
