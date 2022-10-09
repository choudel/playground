fn double(n: i32) -> i32 {
    n * 2
}
fn triple(n: i32) -> i32 {
    n * 3
}

fn make_multiplier(mut x: i32) -> impl FnMut(i32) -> i32 {
    
        move |n|{
            x +=1;
            n * x
        }     
}
fn main() {
    let nums = vec![1, 2, 3];
    let nums_as_iter = nums.into_iter();
    let multiplied = nums_as_iter.map(make_multiplier(2));
    for l in multiplied {
        println!("{}", l)
    }
}
