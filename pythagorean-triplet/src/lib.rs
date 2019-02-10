use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res = HashSet::new();
    for i in 1..sum {
        for j in i..sum {
            for k in j..sum {
                if i + j + k == sum && i.pow(2) + j.pow(2) == k.pow(2) {
                    res.insert([i, j, k]);
                }
            }
        }
    }
    res
}
