pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut out: Vec<String> = vec!();
    if len > digits.len() {
        return out;
    }
    for i in 0..=digits.len()-len {
        out.push(digits.chars().into_iter().skip(i).take(len).collect());
    }
    out
}
