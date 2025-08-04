pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' } as i16;
            let c_val = c as i16 - base;
            let rotated = (c_val + key as i16).rem_euclid(26) + base;
            rotated as u8 as char
        } else {
            c
        }
    }).collect()
}