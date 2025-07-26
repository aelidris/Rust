pub fn delete_and_backspace(s: &mut String) {
    let s_copy = s.clone();
    s.clear();

    let mut escape_index = 0;
    for v in s_copy.chars() {
        if v == '-' {
            s.pop();
        } else if v == '+' {
            escape_index += 1;
        } else if escape_index > 0 {
            escape_index -= 1;
        } else {
            s.push(v);
        }
    }
}

pub fn do_operations(v: &mut [String]) {
    v.iter_mut().for_each(|equation| {
        let (l, r) = equation.split_once(['+', '-']).unwrap();
        let (l, r) = (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap());

        let result = if equation.contains('+') { l + r } else { l - r };
        *equation = result.to_string();
    });
}
