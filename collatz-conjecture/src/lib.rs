pub fn collatz(n: u64) -> Option<u64> {
    let mut i = n;
    let mut v = vec!();
    while i > 1 {
        if i % 2 == 0 {
            i = i / 2;
            v.push(i);
        } else {
            i = (i * 3) + 1;
            v.push(i);
        }
    }
    let len = v.len() as u64;
    match len {
        0 => None,
        _ => Some(len),
    }
}
