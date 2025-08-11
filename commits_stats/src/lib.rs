use json::JsonValue;
use std::collections::HashMap;
use chrono::{DateTime, Datelike};

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut week_counts = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(date_str) = commit["commit"]["committer"]["date"].as_str() {
                if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
                    let week = dt.iso_week().week();
                    let year = dt.year();
                    let week_key = format!("{}-W{}", year, week);
                    *week_counts.entry(week_key).or_insert(0) += 1;
                }
            }
        }
    }

    week_counts
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week: HashMap<String, u32> = HashMap::new();
    for commit in data.members() {
        let count = commits_per_week
            .entry(
                Week(
                    DateTime::parse_from_rfc3339(&commit["commit"]["author"]["date"].to_string())
                        .unwrap()
                        .iso_week(),
                )
                .to_string(),
            )
            .or_insert(0);
        *count += 1;
    }
    commits_per_week
}