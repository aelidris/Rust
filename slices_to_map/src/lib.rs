use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::hash::Hash + std::cmp::Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let min_len = std::cmp::min(keys.len(), values.len());
    let mut map = HashMap::with_capacity(min_len);
    
    for i in 0..min_len {
        map.insert(&keys[i], &values[i]);
    }
    
    map
}