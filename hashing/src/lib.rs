use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let somme: i32 = list.iter().sum();
    let res = (somme as f64) / (list.len() as f64);
    return res;
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();

    let mid = sorted.len() / 2;

    if sorted.len() % 2 != 0 {
        sorted[mid]
    } else {
        (sorted[mid - 1] + sorted[mid]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut frequency = HashMap::new();

    for &num in list {
        *frequency.entry(num).or_insert(0) += 1;
    }

    frequency.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num).unwrap_or(0)
}
