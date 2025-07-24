pub fn factorial(num: u64) -> u64 {
    let mut i = 1;
    let mut fact = 1;
    loop {
        if i > num {
            break fact;
        } else {
            fact = fact * i;
            i += 1;
        }
    }
}