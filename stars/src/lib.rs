pub fn stars(n: u32) -> String {
    let count = 2u32.pow(n);
    "*".repeat(count as usize)
}