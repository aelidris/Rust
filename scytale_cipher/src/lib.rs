pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut res = String::new();
    let mut target = 0;
    let mut initial = 0;
    while res.len() < message.len() {
        res.push(message.chars().nth(target).unwrap());
        if (target + i) < message.len() {target += i;} else {initial+=1; target = initial;} 
    }
    res
}
