pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec!();
    let mut factor = 2;
    let mut num = n;
    while num > 1 {
        if num % factor == 0 {
            num = num / factor;
            factors.push(factor);
        } else {
            factor+=1;
        }
    }
    factors
}
