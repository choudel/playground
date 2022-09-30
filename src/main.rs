fn main() {
    let x =12;
    let y =9;
    let answer=find_answer(x,y);
    println!("the answer is {}", answer)
}
fn find_answer(x:i32,y:i32)-> i32 {
    x*y+5
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_find_answer(){
        let x =7;
        let y =5;
        let answer = find_answer(x, y);
        assert_eq!(40,answer)
    }

}
