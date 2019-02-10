pub fn square(s: u32) -> u64 {
    if s <= 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    let mut grains = 1;
    for _ in 2..=s {
        grains = grains + grains;
    }
    grains
}

pub fn total() -> u64 {
    let mut t = 0;
    for i in 1..=64 {
        t = t + square(i);
    }
    t
}
