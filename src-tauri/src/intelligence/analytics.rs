use std::collections::HashMap;
use crate::types::SearchAnalytics;

/// Record of a single search query.
#[derive(Debug, Clone)]
struct QueryRecord {
    query: String,
    result_count: usize,
    timestamp: String,
    clicked_index: Option<usize>,
}

/// Tracks search analytics: query history, click-through recording, top queries.
///
/// Maintains a ring buffer of max 1000 records (oldest removed when full).
pub struct SearchTracker {
    queries: Vec<QueryRecord>,
    total_searches: u32,
}

impl SearchTracker {
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
            total_searches: 0,
        }
    }

    /// Record a search query and its result count.
    pub fn record_query(&mut self, query: &str, result_count: usize) {
        self.total_searches += 1;

        let timestamp = chrono_now_iso();

        self.queries.push(QueryRecord {
            query: query.to_string(),
            result_count,
            timestamp,
            clicked_index: None,
        });

        // Ring buffer: cap at 1000
        if self.queries.len() > 1000 {
            self.queries.remove(0);
        }
    }

    /// Record a click-through event for the most recent query.
    pub fn record_click(&mut self, clicked_result_index: usize) {
        if let Some(last) = self.queries.last_mut() {
            last.clicked_index = Some(clicked_result_index);
        }
    }

    /// Record a click-through for a specific query by index.
    pub fn record_click_at(&mut self, query_index: usize, clicked_result_index: usize) {
        if let Some(record) = self.queries.get_mut(query_index) {
            record.clicked_index = Some(clicked_result_index);
        }
    }

    /// Get search analytics.
    ///
    /// Returns total searches, top 10 queries by frequency,
    /// and average results per query.
    pub fn get_analytics(&self) -> SearchAnalytics {
        // Top queries by frequency
        let mut query_counts: HashMap<String, usize> = HashMap::new();
        for record in &self.queries {
            *query_counts.entry(record.query.clone()).or_insert(0) += 1;
        }

        let mut sorted: Vec<(String, usize)> = query_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        let top_queries: Vec<String> = sorted
            .into_iter()
            .take(10)
            .map(|(q, _)| q)
            .collect();

        // Average results per query
        let avg_results = if self.queries.is_empty() {
            0.0
        } else {
            let total: usize = self.queries.iter().map(|r| r.result_count).sum();
            total as f64 / self.queries.len() as f64
        };

        SearchAnalytics {
            total_searches: self.total_searches,
            top_queries,
            avg_results_per_query: avg_results,
        }
    }

    /// Get the total number of searches.
    pub fn total_searches(&self) -> u32 {
        self.total_searches
    }
}

impl Default for SearchTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate ISO 8601 timestamp.
fn chrono_now_iso() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs() as i64;
    let days = secs / 86400;
    let time = secs % 86400;
    let h = time / 3600;
    let m = (time % 3600) / 60;
    let s = time % 60;

    let d = days + 719468;
    let era = if d >= 0 { d } else { d - 146096 } / 146097;
    let doe = d - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, h, m, s
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_tracker_new() {
        let tracker = SearchTracker::new();
        assert_eq!(tracker.total_searches(), 0);
    }

    #[test]
    fn test_record_query() {
        let mut tracker = SearchTracker::new();
        tracker.record_query("tax documents", 5);
        assert_eq!(tracker.total_searches(), 1);
    }

    #[test]
    fn test_get_analytics() {
        let mut tracker = SearchTracker::new();
        tracker.record_query("tax documents", 5);
        tracker.record_query("invoices", 3);
        tracker.record_query("tax documents", 7);

        let analytics = tracker.get_analytics();
        assert_eq!(analytics.total_searches, 3);
        assert!(!analytics.top_queries.is_empty());
        assert_eq!(analytics.top_queries[0], "tax documents"); // most frequent
        assert!((analytics.avg_results_per_query - 5.0).abs() < 0.01); // (5+3+7)/3 = 5.0
    }

    #[test]
    fn test_record_click() {
        let mut tracker = SearchTracker::new();
        tracker.record_query("test", 10);
        tracker.record_click(3);

        let analytics = tracker.get_analytics();
        assert_eq!(analytics.total_searches, 1);
    }

    #[test]
    fn test_ring_buffer_cap() {
        let mut tracker = SearchTracker::new();
        for i in 0..1010 {
            tracker.record_query(&format!("query-{}", i), 1);
        }
        assert_eq!(tracker.total_searches(), 1010);
        assert!(tracker.queries.len() <= 1000, "should cap at 1000 records");
    }

    #[test]
    fn test_analytics_empty() {
        let tracker = SearchTracker::new();
        let analytics = tracker.get_analytics();
        assert_eq!(analytics.total_searches, 0);
        assert!(analytics.top_queries.is_empty());
        assert_eq!(analytics.avg_results_per_query, 0.0);
    }
}
