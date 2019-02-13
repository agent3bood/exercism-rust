use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut scores = HashMap::new();
    for c in candidate.chars() {
        if c.is_alphabetic() {
            let char = c.to_lowercase().to_string();
            let val = scores.entry(char).or_insert(0);
            *val += 1;
        }
    }
    for (_, v) in scores {
        if v > 1 {
            return false;
        }
    }
    true
}
