enum Ticket {
    Vip(f64, String),
    Backstage(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Ticket::Vip(260.36, "billy".to_owned()),
        Ticket::Backstage(368.65, "matilda".to_owned()),
        Ticket::Standard(153.26),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::Vip(price, holder) => println!("Vip Holder : {holder:?}, price: {price:?}"),
            Ticket::Standard(price) => println!("Standard ticket:{price:?}"),
            Ticket::Backstage(price, holder) => println!("BS Holder:{holder:?},price:{price:?}"),
        }
    }
}
