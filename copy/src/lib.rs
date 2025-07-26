pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln_abs = (c as f64).abs().ln();
    (c, exp, ln_abs)
}

pub fn str_function(a: String) -> (String, String) {
    let exp = a
        .split_whitespace()
        .map(|n| n.parse::<f64>().expect("REASON").exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    (a, exp)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_abs: Vec<f64> = b
        .iter()
        .map(|&x| (x as f64).abs().ln())
        .collect();

    (b, ln_abs)
}
