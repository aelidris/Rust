pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }
    let n = (c as usize) - ('A' as usize);
    let size = 2 * n + 1;
    let mut diamond = Vec::with_capacity(size);
    for i in 0..=n {
        let mut row = String::with_capacity(size);
        let current_char = (b'A' + (i as u8)) as char;
        row.extend(std::iter::repeat(' ').take(n - i));
        row.push(current_char);
        if i > 0 {
            let spaces = 2 * i - 1;
            row.extend(std::iter::repeat(' ').take(spaces));
            row.push(current_char);
        }
        row.extend(std::iter::repeat(' ').take(n - i));
        diamond.push(row);
    }
    for i in (0..n).rev() {
        diamond.push(diamond[i].clone());
    }
    diamond
}
