use case::CaseExt;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let is_camel_case = compared == compared.to_camel();
    let is_snake_case = compared == compared.to_snake();
    
    if !is_camel_case && !is_snake_case {
        return None;
    }

    let distance = edit_distance(&compared.to_lowercase(), &expected.to_lowercase());
    let expected_len = expected.len();
    
    let similarity = if expected_len == 0 {
        100.0
    } else {
        (1.0 - (distance as f32 / expected_len as f32)) * 100.0
    };

    if similarity > 50.0 {
        Some(format!("{}%", similarity.round() as i32))
    } else {
        None
    }
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let source_len = source_chars.len();
    let target_len = target_chars.len();

    let mut dp = vec![vec![0; target_len + 1]; source_len + 1];

    for i in 0..=source_len {
        dp[i][0] = i;
    }
    for j in 0..=target_len {
        dp[0][j] = j;
    }
    for i in 1..=source_len {
        for j in 1..=target_len {
            let cost = if source_chars[i - 1] == target_chars[j - 1] { 0 } else { 1 };

            dp[i][j] = (dp[i - 1][j] + 1).min(dp[i][j - 1] + 1).min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[source_len][target_len]
}