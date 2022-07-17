
mod not_eratosthenes;
mod eratosthenes;
mod naive;
use std::time::Instant;

struct Cli{
    upper_limit: u32,
    algo: String,
}

fn print_primes(primes: Vec<u32>){
    for value in primes{
        println!("{}",value);
    }
}

fn compare(upper_limit: u32){
    let mut now = Instant::now();
    eratosthenes::find_primes(upper_limit);
    println!("Eratosthenes Milliseconds:{}", now.elapsed().as_millis());
    now = Instant::now();
    not_eratosthenes::find_primes(upper_limit);
    println!("not_eratosthenes Milliseconds:{}", now.elapsed().as_millis());
    now = Instant::now();
    naive::find_primes(upper_limit);
    println!("naive Milliseconds:{}", now.elapsed().as_millis());


}

fn main() {
    let upper_limit = std::env::args().nth(1).expect("No upper limit given!");
    let algo = std::env::args().nth(2).expect("No algorithm specified!");

    let args = Cli {
        upper_limit: upper_limit.trim().parse().unwrap(),
        algo: algo,
    };


    match args.algo.as_str(){
        "compare"=> compare(args.upper_limit),
        "eratosthenes"=> print_primes(eratosthenes::find_primes(args.upper_limit)),
        "not_eratosthenes"=> print_primes(not_eratosthenes::find_primes(args.upper_limit)),
        "naive"=> print_primes(naive::find_primes(args.upper_limit)),
        _=>println!("Invalid algo"),
    }

}
