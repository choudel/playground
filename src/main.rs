use std::collections::HashMap;
#[derive(Debug)]
struct Item {
    name: String,
    in_stock: i32,
}

fn main() {
    let mut store = HashMap::new();
    store.insert(
        1,
        Item {
            name: "Chairs".to_string(),
            in_stock: 5,
        },
    );
    store.insert(
        2,
        Item {
            name: "Beds".to_string(),
            in_stock: 3,
        },
    );
    store.insert(
        3,
        Item {
            name: "Tables".to_string(),
            in_stock: 2,
        },
    );
    store.insert(
        4,
        Item {
            name: "Couches".to_string(),
            in_stock: 0,
        },
    );
    let mut total=0;
    for (k, item) in store.iter() {
        if item.in_stock == 0 {
            println!("Out of stock")
        } else {
            println!("{:?}",item)
        }
        total = total+item.in_stock;
    }
    println!("{total}");

}
