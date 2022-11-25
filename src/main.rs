use std::io;
#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}
pub struct Bills {
    inner: Vec<Bill>,
}
impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }
    fn add(&mut self,bill:Bill){
        self.inner.push(bill);
    }
    fn get_all(&self)->Vec<&Bill>{
        self.inner.iter().collect()
    }
}
mod menu{
use crate::{Bill,Bills,get_input, get_bill_amount};
    pub fn add_bill(bills:&mut Bills){
        println!("Bill name:");
        let name = match get_input() {
            Some(input)=>input,
            None=>return
        };
        let amount = match get_bill_amount() {
            Some(amount)=>amount,
            None=>return
        };
        let bill = Bill{name:name,amount:amount};
        bills.add(bill);
        println!("Bill added")
    }
    pub fn view_bills(bills:&Bills){
        for bill in bills.get_all(){
            println!("{bill:?}");
        }
    }
}
impl Bill {
    pub fn view_bill(&self) {
        println!(
            "the bill is in the name of : {} with the amount of :{}",
            self.name, self.amount
        )
    }
}
pub fn get_input() -> Option<String> {
    let mut buff = String::new();
    while io::stdin().read_line(&mut buff).is_err() {
        println!("please enter your data again")
    }
    let input = buff.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}
pub fn get_bill_amount()->Option<f64>{
    println!("amount:");
    loop {
        let input = match get_input() {
            Some(input)=>input,
            None => return None,
        };
        if &input==""{
            return None;
        } 
        let parsed_input:Result<f64,_> = input.parse();
        match parsed_input{
            Ok(amount)=> return Some(amount),
            Err(_)=>println!("Please enter a number")
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
}
impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None,
        }
    }
    fn show() {
        println!("");
        println!(" == Bill Manager == ");
        println!("1.Add Bill");
        println!("2.View Bills");
        println!("");
        println!("Enter selection:  ")
    }
}
fn main() {
    let mut bills=Bills::new();
    loop {
        MainMenu::show();
        let input = get_input().expect("no data entered");
        match MainMenu::from_str(input.as_str()){
            Some(MainMenu::AddBill)=>menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill)=>menu::view_bills(&bills),
            None=>return,
        }
    }
}
