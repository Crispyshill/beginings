pub fn find_primes(limit: u32) -> Vec<u32>{
let mut list: Vec<u8> = Vec::new();
let upper_limit = f64::sqrt(limit as f64) as u32;

for x in 0..limit{
    if x < 1{
        list.push(0);
    }
    else{
    list.push(1);
    }
}

let mut i = 2;
while i <= upper_limit{
    while list[(i-1) as usize] == 0 && i <= upper_limit{
        //println!("Skipping: {}", i);
        i = i + 1;
    }
    for e in 2..((limit/i)+1){
        //println!("removing: {}, i = {}, e = {}",((i*e)), i, e);
        list[((i*e)-1) as usize] = 0;
    }
    i = i+1;
}


let mut prime_list: Vec<u32> = Vec::new();

for i in 0..list.len(){
    if list[i as usize] == 1{
        prime_list.push((i+1) as u32);
    }
}

prime_list
}
