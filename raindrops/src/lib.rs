pub fn raindrops(n: u32) -> String {
    let mut txt = String::new();
    for i in 1..8 {
        if n % i == 0 {
            if i == 3 {
                txt.push_str("Pling");
            } else if i == 5 {
                txt.push_str("Plang");
            } else if i == 7 {
                txt.push_str("Plong");
            }
        }
    }
    if txt.is_empty() {
        txt.push_str(&n.to_string());
    }
    txt
}
