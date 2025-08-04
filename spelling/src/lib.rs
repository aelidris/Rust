const UNITS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [&str; 3] = [
    "", "thousand", "million",
];

pub fn spell(n: u64) -> String {
    if n == 0 {
        return UNITS[0].to_string();
    }
    let mut chunks = Vec::new();
    let mut num = n;
    let mut scale_index = 0;
    
    while num > 0 {
        let chunk = num % 1000;
        if chunk != 0 {
            let mut chunk_words = convert_less_than_thousand(chunk as usize);
            if scale_index > 0 {
                chunk_words.push_str(" ");
                chunk_words.push_str(SCALES[scale_index]);
            }
            chunks.push(chunk_words);
        }
        num /= 1000;
        scale_index += 1;
    }
    
    chunks.into_iter().rev().collect::<Vec<_>>().join(" ")
}

fn convert_less_than_thousand(n: usize) -> String {
    match n {
        0..=19 => UNITS[n].to_string(),
        20..=99 => {
            let tens = n / 10;
            let units = n % 10;
            if units == 0 {
                TENS[tens].to_string()
            } else {
                format!("{}-{}", TENS[tens], UNITS[units])
            }
        },
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} hundred", UNITS[hundreds])
            } else {
                format!("{} hundred {}", UNITS[hundreds], convert_less_than_thousand(remainder))
            }
        },
        _ => String::new(),
    }
}