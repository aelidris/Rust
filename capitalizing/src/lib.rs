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
    let mut res = String::new();
    let mut test = 1;
    for c in input.split_whitespace() {
        
        for c2 in c.chars() {
            if test == 1 {
                test+=1;
                res.push(c2.to_ascii_uppercase());
                continue
            }
            res.push(c2);
        }
        test = 1;
        res.push(' ')
    }
    res
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