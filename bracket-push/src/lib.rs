pub fn brackets_are_balanced(string: &str) -> bool {
    let mut v = Vec::new();
    for c in string.chars() {
        if c == '{' {
            v.push('{');
        } else if c == '}' {
            match v.last() {
                Some(i) => {
                    if *i == '{' {
                        v.pop();
                    } else {
                        return false;
                    }
                }
                None => return false,
            };
        } else if c == '[' {
            v.push('[');
        } else if c == ']' {
            match v.last() {
                Some(i) => {
                    if *i == '[' {
                        v.pop();
                    } else {
                        return false;
                    }
                }
                None => return false,
            };
        } else if c == '(' {
            v.push('(');
        } else if c == ')' {
            match v.last() {
                Some(i) => {
                    if *i == '(' {
                        v.pop();
                    } else {
                        return false;
                    }
                }
                None => return false,
            };
        }
    }
    v.len() == 0
}
