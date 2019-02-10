use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let tupni = UnicodeSegmentation::graphemes(input, true).rev()
    .flat_map(|g| g.chars())
    .collect();
    tupni
}
