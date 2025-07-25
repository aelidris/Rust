pub fn is_empty(v: &str) -> bool {
    if v.len() == 0 {
        return true
    } else {
        return false
    }
}

pub fn is_ascii(v: &str) -> bool {
    if v.len() == v.chars().count() {
        return true
    } else {
        return false
    }
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("not found")
}