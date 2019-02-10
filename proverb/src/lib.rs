pub fn build_proverb(list: &[&str]) -> String {
    let mut out = String::new();
    if list.is_empty() {
        return out;
    }
    let len = list.len() - 1;
    for i in 0..len {
         out = format!("{}For want of a {} the {} was lost.\n", out, list[i], list[i+1]);
    }
    format!("{}And all for the want of a {}.", out, list[0])
}
