#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}
trait Priority {
    fn get_priority(&self) -> ServicePriority;
}
#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}
#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}
fn print_priority<T: Priority+std::fmt::Debug>(prio: T) {
    let result = prio.get_priority();
    println!("name is {:?} the priority is {:?}", prio, result);
}
fn main() {
    let mathiew = Guest;
    print_priority(mathiew);
}
