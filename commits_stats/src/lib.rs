use json::JsonValue;
use std::collections::HashMap;
use chrono::{DateTime, Datelike};

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author: HashMap<String, u32> = HashMap::new();
    for commit in data.members() {
        let count = commits_per_author
            .entry(commit["author"]["login"].to_string())
            .or_insert(0);
        *count += 1;
    }
    commits_per_author
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