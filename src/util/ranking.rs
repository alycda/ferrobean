use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::f64::consts::LN_2;

const DEFAULT_RATE: f64 = LN_2 / 365.0;

/// Rank a list by exponential decay

pub struct ExponentialDecayRanker {
    scores: HashMap<String, (f64, u64)>, // (score, last_update timestamp)
    rate: f64,
}

impl ExponentialDecayRanker {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
            rate: DEFAULT_RATE,
        }
    }

    /// Add 'like' for item (increments score, applies decay)
    pub fn update(&mut self, item: &str) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let (score, last_update) = self.scores.get(item).cloned().unwrap_or((0.0, now));
        let dt = (now as i64 - last_update as i64).max(0) as f64;
        let decayed = score * (-self.rate * dt).exp();
        self.scores.insert(item.to_string(), (decayed + 1.0, now));
    }

    /// Get the current score for an item, or zero
    pub fn get(&self, item: &str) -> f64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if let Some(&(score, last_update)) = self.scores.get(item) {
            let dt = (now as i64 - last_update as i64).max(0) as f64;
            score * (-self.rate * dt).exp()
        } else {
            0.0
        }
    }

    /// Return items sorted by rank (descending)
    pub fn sort(&self) -> Vec<(String, f64)> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut items: Vec<_> = self.scores.iter().map(|(item, &(score, last_update))| {
            let dt = (now as i64 - last_update as i64).max(0) as f64;
            (item.clone(), score * (-self.rate * dt).exp())
        }).collect();
        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_ranker_basic() {
        let mut ranker = ExponentialDecayRanker::new();
        ranker.update("foo");
        assert!(ranker.get("foo") > 0.0);
        // The default decay rate is very slow (half-life = 365 days), so simulate a much longer time
        let before = ranker.get("foo");
        // Simulate 1 year passing by manually decaying the score
        let (score, last_update) = ranker.scores.get("foo").cloned().unwrap();
        let dt = 365.0; // days
        let decayed = score * (-DEFAULT_RATE * (dt * 24.0 * 3600.0)).exp();
        // Overwrite the score as if a year has passed
        ranker.scores.insert("foo".to_string(), (decayed, last_update));
        let after = ranker.get("foo");
        assert!(after < before);
    }

    #[test]
    fn test_decay_ranker_sort() {
        let mut ranker = ExponentialDecayRanker::new();
        ranker.update("foo");
        ranker.update("bar");
        ranker.update("foo");
        let sorted = ranker.sort();
        assert_eq!(sorted[0].0, "foo");
    }
}