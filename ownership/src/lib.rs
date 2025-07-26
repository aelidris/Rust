pub fn first_subword(mut s: String) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            result.push(c);
            continue;
        }
        if (i>0) && (c.is_lowercase()) {
            result.push(c);
        } else if !c.is_lowercase() {
            break;
        }
    }
    if result.len() > 0 {
        return result;
    }
    s
}