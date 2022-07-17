pub fn find_primes(num: u32) -> Vec<u32>{
let mut list: Vec<u8> = Vec::new();
for _ in 0..num{
    list.push(1);
}
for x in 0..num{
    for y in 2..(x/2)+2{
        //println!("x = {}, y = {}",x,y);
        if(x+1)%y == 0{
            list[(x) as usize] = 0;
            break;
        }
    }
}

let mut primes: Vec<u32> = Vec::new();

for x in 0..list.len(){
    if list[x as usize] == 1{
        primes.push((x+1) as u32);
    }
}
primes
}
