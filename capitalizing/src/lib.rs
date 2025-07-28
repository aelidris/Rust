pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            let t = c.to_ascii_uppercase();
            res.push(t);
            continue;
        }
        res.push(c);
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();

    for c in input.chars() {
        if c.is_uppercase() {
            res.push(c.to_ascii_lowercase());
        } else if c.is_lowercase() {
            res.push(c.to_ascii_uppercase());
        } else {
            res.push(c);
        }
    }

    res
}