struct ClassicCars{
  make:&'static str,
  models:Vec<(&'static str,i32)>
}
impl ClassicCars{
  fn smart_get<F>(&self,f:F)
  where 
  F:Fn(&Vec<(&'static str,i32)>)
  {
    f(&self.models)
  }
}
fn main() {
    let car_collection = vec![("Thunder",1960),("CObra",1966),("GT",1967)];
    let ford_models = ClassicCars{
      make:"Ford",
      models:car_collection
    };
    ford_models.smart_get(|x|{
      let res:Vec<&(&str, i32)> = x.into_iter().filter(|x|x.1>1960).collect();
      println!("{:?}",res);
    })
}
