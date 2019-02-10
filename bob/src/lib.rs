pub fn reply(message: &str) -> &str {
    let answer = "Sure.";
    let chill = "Whoa, chill out!";
    let calm = "Calm down, I know what I'm doing!";
    let fine = "Fine. Be that way!";
    let whatever = "Whatever.";

    if is_question(&message) && is_yell(&message) {
        calm
    } else if is_address(&message) {
        fine
    } else if is_yell(&message) {
        chill
    } else if is_question(&message) {
        answer
    } else {
        whatever
    }
}

fn is_question(message: &str) -> bool {
    for c in message.chars().rev() {
        if c == '?' {
            return true;
        } else if c.is_whitespace() {
            continue;
        } else {
            return false;
        }
    }
    false
}

fn is_yell(message: &str) -> bool {
    let mut cap = 0;
    let mut small = 0;
    for c in message.chars() {
        if c.to_uppercase().next() == Some(c) && c.is_alphabetic() {
            cap+=1;
        } else if c.is_alphabetic() {
            small+=1;
        } else { continue; }
    }
    cap > small
}

fn is_address(message: &str) -> bool {
    for c in message.chars() {
        if !c.is_whitespace() {
            return false;
        } else {
            continue;
        }
    }
    true
}
