pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let mut initials = String::new();
            for word in name.split_whitespace() {
                if let Some(first_char) = word.chars().next() {
                    initials.push(first_char);
                    initials.push('.');
                }
            }
            initials
        })
        .collect()
}
