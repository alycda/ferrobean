//! Ranking utilities

use std::collections::HashMap;
use time::Date;

const ZERO: f64 = 0.0;
const DEFAULT_RATE: f64 = std::f64::consts::LN_2 / 365.0;

/// Rank a list by exponential decay.
///
/// Maintains scores for the items in a list. We can think of this as the sum
/// of all 'likes', where the value of a 'like' starts at 1 and decays
/// exponentially. So the current score would be given by (where `t` is the
/// current time and `l` is the time of the 'like')
///
///     s = Σ exp(-RATE * (t - l))
///
/// As only the relative order on the items is relevant, we can multiply all
/// scores by exp(RATE * t) and so we need to compute the following
/// score:
///
///     s = Σ exp(RATE * l)
///
/// To avoid huge numbers, we actually compute and store the logarithm of that
/// sum.
///
/// # Arguments
///
/// * `list` - If given, this list is ranked by `.sort()` otherwise all
///   items with at least one 'like' will be ranked.
/// * `rate` - This sets the rate of decay. `1/rate` will be the time (in
///   days) that it takes for the value of a 'like' to decrease by
///   `1/e`. The default rate is set to `ln(2) / 365` so
///   that a 'like' from a year ago will count half as much as one from
///   today.
pub struct ExponentialDecayRanker {
    list: Option<Vec<String>>,
    rate: f64,
    scores: HashMap<String, f64>,
}

impl Default for ExponentialDecayRanker {
    fn default() -> Self {
        Self::new()
    }
}

impl ExponentialDecayRanker {
    /// Create a new ExponentialDecayRanker with default rate
    pub fn new() -> Self {
        Self {
            list: None,
            rate: DEFAULT_RATE,
            scores: HashMap::new(),
        }
    }

    /// Create a new ExponentialDecayRanker with a specific list
    pub fn new_with_list(list: Vec<String>) -> Self {
        Self {
            list: Some(list),
            rate: DEFAULT_RATE,
            scores: HashMap::new(),
        }
    }

    /// Create a new ExponentialDecayRanker with a specific rate
    pub fn new_with_rate(rate: f64) -> Self {
        Self {
            list: None,
            rate,
            scores: HashMap::new(),
        }
    }

    /// Create a new ExponentialDecayRanker with both list and rate
    pub fn new_with_list_and_rate(list: Vec<String>, rate: f64) -> Self {
        Self {
            list: Some(list),
            rate,
            scores: HashMap::new(),
        }
    }

    /// Add 'like' for item.
    ///
    /// # Arguments
    ///
    /// * `item` - An item in the list that is being ranked.
    /// * `date` - The date on which the item has been liked.
    pub fn update(&mut self, item: &str, date: Date) {
        let score = self.get(item);
        let time = date.to_julian_day() as f64;
        let higher = score.max(time * self.rate);
        let lower = score.min(time * self.rate);
        self.scores.insert(
            item.to_string(),
            higher + (lower - higher).exp().ln_1p(),
        );
    }

    /// Get the current score for an item, or zero.
    pub fn get(&self, item: &str) -> f64 {
        self.scores.get(item).copied().unwrap_or(ZERO)
    }

    /// Return items sorted by rank.
    pub fn sort(&self) -> Vec<String> {
        if let Some(ref list) = self.list {
            let mut sorted_list = list.clone();
            sorted_list.sort_by(|a, b| {
                self.get(b).partial_cmp(&self.get(a)).unwrap_or(std::cmp::Ordering::Equal)
            });
            sorted_list
        } else {
            let mut items: Vec<String> = self.scores.keys().cloned().collect();
            items.sort_by(|a, b| {
                self.get(b).partial_cmp(&self.get(a)).unwrap_or(std::cmp::Ordering::Equal)
            });
            items
        }
    }

    /// Return items with their scores, sorted by rank.
    pub fn sort_with_scores(&self) -> Vec<(String, f64)> {
        let items = if let Some(ref list) = self.list {
            list.clone()
        } else {
            self.scores.keys().cloned().collect()
        };

        let mut scored_items: Vec<(String, f64)> = items
            .into_iter()
            .map(|item| {
                let score = self.get(&item);
                (item, score)
            })
            .collect();

        scored_items.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        scored_items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_basic_functionality() {
        let mut ranker = ExponentialDecayRanker::new();
        
        let date1 = date!(2023-01-01);
        let date2 = date!(2023-06-01);
        
        ranker.update("item1", date1);
        ranker.update("item2", date2);
        ranker.update("item1", date2); // item1 gets another like
        
        let sorted = ranker.sort();
        assert_eq!(sorted[0], "item1"); // item1 should be first due to more recent activity
    }

    #[test]
    fn test_with_predefined_list() {
        let list = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
        let mut ranker = ExponentialDecayRanker::new_with_list(list);
        
        let date = date!(2023-01-01);
        ranker.update("banana", date);
        
        let sorted = ranker.sort();
        assert_eq!(sorted[0], "banana");
        assert!(sorted.contains(&"apple".to_string()));
        assert!(sorted.contains(&"cherry".to_string()));
    }

    #[test]
    fn test_get_score() {
        let mut ranker = ExponentialDecayRanker::new();
        assert_eq!(ranker.get("nonexistent"), 0.0);
        
        let date = date!(2023-01-01);
        ranker.update("item1", date);
        assert!(ranker.get("item1") > 0.0);
    }
}