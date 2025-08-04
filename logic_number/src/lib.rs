pub fn number_logic(num: u32) -> bool {
    let convert = num.to_string();
    let mut sum = 0;
    let pow = convert.len();
    for c in convert.chars() {
        sum += power(c, pow);
    }
    if sum == num {
        return true;
    }
    return false;
}

pub fn power(ch: char, p: usize) -> u32 {
    ch.to_digit(10).unwrap().pow(p.try_into().unwrap())
}
