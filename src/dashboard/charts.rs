use std::collections::HashMap;

use crate::task::models::Task;

/// Get complexity distribution from DB tasks
pub fn get_complexity_distribution(tasks: &[Task]) -> Vec<(String, usize)> {
    let mut dist: HashMap<String, usize> = HashMap::new();

    for task in tasks {
        if let Some(ref tc) = task.time_complexity {
            let key = normalize_complexity(tc);
            *dist.entry(key).or_insert(0) += 1;
        }
    }

    if dist.is_empty() {
        // Show empty categories
        return vec![
            ("O(1)".into(), 0),
            ("O(log n)".into(), 0),
            ("O(n)".into(), 0),
            ("O(n log n)".into(), 0),
            ("O(n²)".into(), 0),
        ];
    }

    let mut result: Vec<(String, usize)> = dist.into_iter().collect();
    result.sort_by(|a, b| {
        complexity_order(&a.0).cmp(&complexity_order(&b.0))
    });
    result
}

/// Get category progress (solved/total per category)
pub fn get_category_progress(tasks: &[Task]) -> Vec<(String, usize, usize)> {
    let mut categories: HashMap<String, (usize, usize)> = HashMap::new();

    for task in tasks {
        if let Some(ref cat) = task.category {
            let entry = categories.entry(cat.clone()).or_insert((0, 0));
            entry.1 += 1; // total
            if task.status == "completed" {
                entry.0 += 1; // solved
            }
        }
    }

    if categories.is_empty() {
        return vec![("No categories yet".into(), 0, 0)];
    }

    let mut result: Vec<(String, usize, usize)> = categories
        .into_iter()
        .map(|(cat, (solved, total))| (cat, solved, total))
        .collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

fn normalize_complexity(s: &str) -> String {
    let s = s.to_lowercase().replace(' ', "");
    if s.contains("n^2") || s.contains("n²") {
        "O(n²)".into()
    } else if s.contains("nlogn") || s.contains("n*logn") {
        "O(n log n)".into()
    } else if s.contains("logn") {
        "O(log n)".into()
    } else if s.contains("2^n") {
        "O(2ⁿ)".into()
    } else if s.contains('n') {
        "O(n)".into()
    } else if s.contains('1') {
        "O(1)".into()
    } else {
        s
    }
}

fn complexity_order(s: &str) -> u8 {
    match s {
        "O(1)" => 0,
        "O(log n)" => 1,
        "O(n)" => 2,
        "O(n log n)" => 3,
        "O(n²)" => 4,
        "O(2ⁿ)" => 5,
        _ => 10,
    }
}
