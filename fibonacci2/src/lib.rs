pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut i = 2;

    while i <= n {
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    }

    b
}
