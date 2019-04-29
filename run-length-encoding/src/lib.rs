pub fn encode(source: &str) -> String {
    let mut out= String::from("");
    let mut counter = -1;
    let mut prev_char :Option<char> = None;
    for c in source.chars() {
        counter+=1;
        if prev_char != Some(c) && prev_char != None {
            if counter > 1
            {
                out.push_str(&counter.to_string());
            }
            counter = 0;
            out.push(prev_char.unwrap());
        }
        prev_char = Some(c);
    }
    // after the loop append the last char
    counter+=1;
    if counter > 1
    {
        out.push_str(&counter.to_string());
    }
    counter = 0;
    match prev_char {
        None => {},
        Some(c) => out.push(c),
    }
    out
}

pub fn decode(source: &str) -> String {
    const RADIX: u32 = 10;
    let mut out = String::from("");
    let mut counter = 0;
    for c in source.chars() {
        if c.is_numeric() {
            counter*=10;
            counter+= c.to_digit(RADIX).unwrap_or(0);
        } else {
            out.push(c);
            while counter > 1 {
                out.push(c);
                counter-=1;
            }
            counter = 0;
        }
    }
    out
}
