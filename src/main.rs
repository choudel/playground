use std::collections::HashMap;

fn main() {
    let mut grid = HashMap::new();
    grid.insert((2, 6), "three");
    grid.insert((4, 7), "rock");
    grid.entry((4,7)).or_insert("empty");
    grid.remove(&(4,7));

        let coords = (2,6);
        if let Some(cell)=grid.get(&coords){
            println!("{}: {:?}", cell, coords)
        }else{
            println!("nothig in {:?}",coords)
        }
    for (key, value) in &grid {
        println!("{}: {:?}", value, key)
    }
}
