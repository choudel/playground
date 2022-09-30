fn main() {
    let n = 42;
    println!("Fib({})={}", n, fib(n));
}
fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut x = 2;
        let mut a = 0;
        let mut b = 1;
        loop {
            let next = a + b;
            a = b;
            b = next;
            x += 1;
            if x >= n {
                break;
            }
        }
        b
    }
}
