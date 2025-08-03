pub fn is_pangram(s: &str) -> bool {
    let mut simplify: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    simplify.sort();
    simplify.dedup();
    simplify.len() == 26
}
