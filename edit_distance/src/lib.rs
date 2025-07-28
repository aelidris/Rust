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
