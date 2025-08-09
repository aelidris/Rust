pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|num_str| {
            if num_str.ends_with('k') {
                let num = num_str.trim_end_matches('k').parse::<f32>().unwrap_or(0.0);
                Box::new((num * 1000.0) as u32)
            } else {
                Box::new(num_str.parse::<u32>().unwrap_or(0))
            }
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter()
        .map(|boxed| *boxed)
        .collect()
}
