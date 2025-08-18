pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 0 || n == 1 {
        return vec![];
    }

    let mut prefix = vec![1; n];
    let mut suffix = vec![1; n];
    let mut result = vec![1; n];

    for i in 1..n {
        prefix[i] = prefix[i - 1] * arr[i - 1];
    }

    for i in (0..n - 1).rev() {
        suffix[i] = suffix[i + 1] * arr[i + 1];
    }

    for i in 0..n {
        result[i] = prefix[i] * suffix[i];
    }
    result
}