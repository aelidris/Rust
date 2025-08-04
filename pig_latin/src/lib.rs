pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = text.chars().peekable();
    let mut consonant_cluster = String::new();
    if let Some(&first) = chars.peek() {
        if vowels.contains(&first.to_ascii_lowercase()) {
            return format!("{}ay", text);
        }
    }
    while let Some(&c) = chars.peek() {
        let lowercase_c = c.to_ascii_lowercase();
        if !vowels.contains(&lowercase_c) {
            consonant_cluster.push(c);
            chars.next();

            if lowercase_c == 'q' && chars.peek() == Some(&'u') {
                consonant_cluster.push('u');
                chars.next();
                break;
            }
        } else {
            break;
        }
    }
    let remaining: String = chars.collect();
    format!("{}{}ay", remaining, consonant_cluster)
}
