use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut n = 0;
    for (_key, value) in h {
        if value > n {
            n = value;
        }
    }
    n
}