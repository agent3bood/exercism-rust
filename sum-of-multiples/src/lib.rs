pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    let mut multiples = vec![];
    for i in factors {
        for j in 1..=limit {
            let n = j * i;
            if n < limit {
                if !multiples.contains(&n) {
                    multiples.push(n);
                }
            }
        }
    }
    println!("{:?}", multiples);
    // now sum it up
    for i in multiples {
        sum = sum + i;
    }
    // return
    sum
}
