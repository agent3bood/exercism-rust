pub fn nth(n: u32) -> u32 {
    let mut prime_index = 0;
    let mut i = 2;
    while prime_index <= n {
        //println!("{}", i);
        if is_prime(i) {
            prime_index+=1;
            //println!("prime:{} index:{}", i, prime_index);
            if prime_index == n {
                break;
            }
        } else {
            i+=1;
            continue;
        }
        i+=1;
    }
    i
}

fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        } else {
            continue;
        }
    }
    true
}
