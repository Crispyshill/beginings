pub fn find_primes(num: u32) -> Vec<u32>{
    let mut primes = Vec::new();
    let upper_limit = f64::sqrt(num as f64) as u32;

    for x in 1..num{
        primes.push(x);
    }
    // Loops
    for x in 2..(upper_limit){
        for y in 2..(num/x)+1{
            primes.retain(|value| *value != (x*y));
        }
    }

    primes
}
