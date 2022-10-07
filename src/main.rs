

fn main() {
    let primes = vec![2,3,5];
    let mut i =0;
    while i < primes.len(){
        println!("Prime #{}: {}",i+1 , primes[i]);
        i += 1;
    }
    let mut primes_iter = primes.iter().enumerate();
    while let Some((i,prime))=primes_iter.next(){
        println!("Prime is #{} : {}",i+1, prime);
    }
    for (i,prime)in primes.iter().enumerate(){
        println!("Prime is #{} : {}",i+1, prime);
    }
}
