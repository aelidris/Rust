pub fn rotate(input: &str, key: i8) -> String {
    let mut ro = key;
    if key < 0 {
        ro = key * -1 + 24;
    }
    let all = "abcdefghijklmnopqrstuvwxyz".to_string();
    let all_v2 = all.to_uppercase();
    let mut res = String::new();
    let mut new_index;
    for chaaar in input.chars() {
        new_index = (helper(chaaar) + (ro as usize)) % 26;
        if chaaar.is_ascii_lowercase() {
            res.push(all.chars().nth(new_index).unwrap());
        } else if chaaar.is_ascii_uppercase() {
            res.push(all_v2.chars().nth(new_index).unwrap());
        } else {
            res.push(chaaar);
        }
    }
    res
}

pub fn helper(ch: char) -> usize {
    let mut all = String::new();
    if ch.is_lowercase() {
        all = "abcdefghijklmnopqrstuvwxyz".to_string();
    } else if ch.is_uppercase() {
        all = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    }
    let mut res = 0;
    for (i, c) in all.chars().enumerate() {
        if c == ch {
            res = i;
            break;
        }
    }
    return res;
}
