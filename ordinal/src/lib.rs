pub fn num_to_ordinal(x: u32) -> String {
    let mut c = x.to_string();
    match x % 100 {
        11 | 12 | 13 => c.push_str("th"),
        _ => match x % 10 {
            1 => c.push_str("st"),
            2 => c.push_str("nd"),
            3 => c.push_str("rd"),
            _ => c.push_str("th"),
        },
    }
    c
}