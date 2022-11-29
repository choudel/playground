#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}
#[derive(Debug, PartialEq, Eq,PartialOrd, Ord)]
enum City {
    Barland,
    Bazopolis,
    Fooville,
}
#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}
impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self {
            name: name.to_string(),
            age,
            city,
        }
    }
}
fn new_ids() -> Cards {
    Cards { inner: vec![
        IdCard::new("Amy",1,City::Fooville),
        IdCard::new("Matt",5,City::Barland),
        IdCard::new("Bailee",54,City::Bazopolis),
        IdCard::new("Anthoine",30,City::Bazopolis),
    ] }
}
#[derive(Debug)]
struct YoungPeople<'a>{
inner:Vec<&'a IdCard>
}
impl <'a> YoungPeople<'a> {
    fn living_in_fooville(&self)->Self{
        Self { inner: self
        .inner.iter()
        .filter(|id| id.city == City::Fooville)
        .map(|id|*id)
        .collect()
        }
    }
}
fn main() {
    let ids = new_ids();
    let young = YoungPeople{
        inner: ids.inner.iter().filter(|id| id.age <= 20).collect(),
    };
    for id in ids.inner.iter(){
        println!("{id:?}**")
    }
    for id in young.inner.iter(){
        println!("{id:?}--")
    }
    for id in young.living_in_fooville().inner.iter(){
        println!("{id:?}11")
    }
}
