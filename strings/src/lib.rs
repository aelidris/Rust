pub fn char_length(s: &str) -> usize {
    let mut i = 0;
    for _c in s.chars() {
        i += 1;
    }
    return i
}