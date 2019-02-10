pub fn private_key(p: u64) -> u64 {
    let prime = 1;
    for i in 2..p {
        if is_prime(i) {
            return i;
        }
    }
    prime
}

fn is_prime(p: u64) -> bool {
    let mut is = true;
    for i in 2..p {
        if p % i == 0 {
            is = false;
        }
    }
    is
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow(b_pub, a, p)
}

fn pow(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut res = 1u64;
    a = a % p;
    while b > 0 {
        if b % 2 != 0 {
            res = (res * a) % p;
        }
        b /= 2;
        a = (a * a) % p;
    }
    res
}
