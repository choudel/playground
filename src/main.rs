use std::io;
enum PowerState {
    Off,
    Hibernate,
    Sleep,
    Reboot,
}
impl PowerState {
    fn new(input: &str) -> Option<PowerState> {
        let recieved = input.trim().to_lowercase();
        match recieved.as_str() {
            "off" => Some(PowerState::Off),
            "hibernate" => Some(PowerState::Hibernate),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            _ => None,
        }
    }
}
fn print_action(state:PowerState){
  use PowerState::*;
  match state{
    Off=>println!("Offing"),
    Hibernate=>println!("Zzzzz"),
    Sleep=>println!("slpzzz"),
    Reboot=>println!("bootex"),
  }
}
fn main() {
  let mut buff = String::new();
  let state =io::stdin().read_line(&mut buff);
  if state.is_ok(){
    match PowerState::new(&buff){
      Some(state)=> print_action(state),
      None=>println!("invalid state" )
    }
  }else {
    println!("something went wrong")
  }

}
