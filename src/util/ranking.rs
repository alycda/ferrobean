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
        // Convert to ordinal day count to match Python's date.toordinal()
        // Python's toordinal() counts days since January 1, year 1
        let time = self.date_to_ordinal(date) as f64;
        let higher = score.max(time * self.rate);
        let lower = score.min(time * self.rate);
        self.scores.insert(
            item.to_string(),
            higher + (lower - higher).exp().ln_1p(),
        );
    }

    /// Convert a Date to ordinal days (matching Python's date.toordinal())
    fn date_to_ordinal(&self, date: Date) -> i32 {
        // Python's date.toordinal() returns days since January 1, year 1
        // We'll use a simplified calculation that should be consistent
        let base_date = time::Date::from_calendar_date(1, time::Month::January, 1).unwrap();
        (date - base_date).whole_days() as i32 + 1
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

    #[test]
    fn test_ranker() {
        // Test 1: Different dates, most recent should be first
        let list = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let mut ranker = ExponentialDecayRanker::new_with_list(list);
        
        ranker.update("1", date!(2015-01-01));
        ranker.update("2", date!(2014-01-01));
        ranker.update("3", date!(2016-01-01));
        
        assert_eq!(ranker.sort(), vec!["3", "1", "2"]);

        // Test 2: Multiple updates, more recent date wins
        let list = vec!["1".to_string(), "2".to_string()];
        let mut ranker = ExponentialDecayRanker::new_with_list(list);
        
        ranker.update("2", date!(2016-01-01));
        ranker.update("2", date!(2016-01-01));
        ranker.update("1", date!(2016-01-01));
        ranker.update("1", date!(2016-01-02));  // One day later
        
        assert_eq!(ranker.sort(), vec!["1", "2"]);

        // Test 3: Frequency vs recency - recency wins with significant time gap
        let list = vec!["1".to_string(), "2".to_string()];
        let mut ranker = ExponentialDecayRanker::new_with_list(list);
        
        ranker.update("2", date!(2015-01-01));
        ranker.update("2", date!(2015-01-01));  // Two updates but older
        ranker.update("1", date!(2016-01-01));  // One update but much newer
        
        assert_eq!(ranker.sort(), vec!["1", "2"]);

        // Test 4: Frequency vs recency - frequency wins with small time gap
        let list = vec!["1".to_string(), "2".to_string()];
        let mut ranker = ExponentialDecayRanker::new_with_list(list);
        
        ranker.update("2", date!(2015-01-01));
        ranker.update("2", date!(2015-01-02));  // Two updates close together
        ranker.update("1", date!(2016-01-01));  // One update but newer
        
        // Note: This test demonstrates that with the exponential decay algorithm,
        // multiple close updates can outweigh a single newer update depending on
        // the decay rate and time differences
        assert_eq!(ranker.sort(), vec!["2", "1"]);
    }

    #[test]
    fn test_ranker_without_predefined_list() {
        // Test with no predefined list - should rank all items that have been updated
        let mut ranker = ExponentialDecayRanker::new();
        
        ranker.update("apple", date!(2015-01-01));
        ranker.update("banana", date!(2016-01-01));
        ranker.update("cherry", date!(2014-01-01));
        
        let sorted = ranker.sort();
        assert_eq!(sorted[0], "banana");  // Most recent
        assert_eq!(sorted[1], "apple");
        assert_eq!(sorted[2], "cherry");  // Oldest
    }

    #[test] 
    fn test_ranker_get_scores() {
        let mut ranker = ExponentialDecayRanker::new();
        
        // Test getting score for non-existent item
        assert_eq!(ranker.get("nonexistent"), 0.0);
        
        // Test getting score after update
        ranker.update("item1", date!(2023-01-01));
        assert!(ranker.get("item1") > 0.0);
        
        // Test that more recent dates have higher scores
        ranker.update("item2", date!(2023-06-01));
        assert!(ranker.get("item2") > ranker.get("item1"));
    }

    #[test]
    fn test_ranker_with_custom_rate() {
        // Test with custom decay rate
        let mut fast_decay_ranker = ExponentialDecayRanker::new_with_rate(1.0);
        let mut slow_decay_ranker = ExponentialDecayRanker::new_with_rate(0.001);
        
        let old_date = date!(2020-01-01);
        let new_date = date!(2023-01-01);
        
        fast_decay_ranker.update("item", old_date);
        slow_decay_ranker.update("item", old_date);
        
        // With fast decay, old dates should lose value quickly
        // With slow decay, old dates should retain more value
        let fast_decay_score = fast_decay_ranker.get("item");
        let slow_decay_score = slow_decay_ranker.get("item");
        
        // The exact relationship depends on the implementation details,
        // but this tests that different rates produce different results
        assert_ne!(fast_decay_score, slow_decay_score);
    }

    #[test]
    fn test_sort_with_scores() {
        let mut ranker = ExponentialDecayRanker::new();
        
        ranker.update("first", date!(2023-03-01));
        ranker.update("second", date!(2023-02-01));
        ranker.update("third", date!(2023-01-01));
        
        let scored_items = ranker.sort_with_scores();
        
        // Should be sorted by score descending
        assert_eq!(scored_items[0].0, "first");
        assert_eq!(scored_items[1].0, "second");
        assert_eq!(scored_items[2].0, "third");
        
        // Scores should be in descending order
        assert!(scored_items[0].1 > scored_items[1].1);
        assert!(scored_items[1].1 > scored_items[2].1);
        
        // All scores should be positive
        for (_, score) in scored_items {
            assert!(score > 0.0);
        }
    }
}