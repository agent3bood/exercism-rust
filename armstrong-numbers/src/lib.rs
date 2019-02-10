pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let len = str.len() as u32;
    let mut sum = 0;
    for c in str.chars() {
        let digit = c.to_digit(10).unwrap();
        sum = sum + digit.pow(len);
    }
    sum == num
}
