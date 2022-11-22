fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}
fn div(a:i32,b:i32)->Option<i32>{
    if b==0{
        None
    }else{
        Some(a/b)
    }
     
}
fn concat(first:&str , second:&str)->String{
    format!("{} {}",first,second)
}
fn main() {}
#[cfg(test)]
mod test {

    use crate::{clamp, div,concat};

    #[test]
    fn check_clamp_outside(){
        let result =clamp(6, 3, 5);
        let expected = 5;
        assert_eq!(result,expected,"no that's not it")
    }
    #[test]
    fn check_clamp_middle(){
        let result =clamp(4, 3, 5);
        let expected = 4;
        assert_eq!(result,expected,"no that's not it")
    }
    #[test]
    fn check_div(){
        let result = div(6, 1);
        let expected = Some(6);
        assert_eq!(result,expected,"no that")
    }
    #[test]
    fn check_div_zero(){
        let result = div(6, 0);
        let expected = None;
        assert_eq!(result,expected,"no that")
    }
   
    #[test]
    fn check_test(){
        let result= concat("a","b");
        let expected=String::from("a b");
        assert_eq!(result,expected,"should be placed a space")
    }
}