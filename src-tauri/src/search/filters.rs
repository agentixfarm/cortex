use std::collections::HashSet;
use regex::Regex;

use crate::engine::CortexEngine;
use crate::error::AppError;
use crate::types::SearchFilters;

/// Parsed entity filter extracted from natural language query.
#[derive(Debug, Clone)]
pub struct EntityFilter {
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub before_date: Option<String>,
    pub after_date: Option<String>,
}

/// Apply metadata filters to narrow the candidate set before vector search.
///
/// If all filter fields are None, returns None (no filtering — search all).
/// Otherwise, iterates the collection and intersects matching doc IDs.
pub fn apply_metadata_filters(
    filters: &SearchFilters,
    engine: &CortexEngine,
) -> Result<Option<HashSet<String>>, AppError> {
    // If all filters are None, skip filtering
    if filters.doc_type.is_none()
        && filters.space_id.is_none()
        && filters.date_from.is_none()
        && filters.date_to.is_none()
        && filters.tags.is_none()
    {
        return Ok(None);
    }

    let collection_arc = engine
        .collections
        .get_collection("documents_384")
        .ok_or_else(|| AppError::VectorStorage("documents_384 collection not found".to_string()))?;

    let collection = collection_arc.read();
    let all_ids = collection
        .db
        .keys()
        .map_err(|e| AppError::VectorStorage(e.to_string()))?;

    let mut result_set: HashSet<String> = all_ids.into_iter().collect();

    for id in result_set.clone() {
        let entry = collection
            .db
            .get(&id)
            .map_err(|e| AppError::VectorStorage(e.to_string()))?;

        let matches = match entry {
            Some(entry) => {
                let metadata = entry.metadata.as_ref();
                let mut pass = true;

                if let Some(ref doc_type) = filters.doc_type {
                    let stored = metadata
                        .and_then(|m| m.get("doc_type"))
                        .and_then(|v| v.as_str());
                    if stored != Some(doc_type.as_str()) {
                        pass = false;
                    }
                }

                if pass {
                    if let Some(ref space_id) = filters.space_id {
                        let stored = metadata
                            .and_then(|m| m.get("space_ids"))
                            .and_then(|v| v.as_array());
                        let in_space = stored
                            .map(|arr| {
                                arr.iter()
                                    .any(|v| v.as_str() == Some(space_id.as_str()))
                            })
                            .unwrap_or(false);
                        if !in_space {
                            pass = false;
                        }
                    }
                }

                if pass {
                    if let Some(ref date_from) = filters.date_from {
                        let stored = metadata
                            .and_then(|m| m.get("created_at"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if stored < date_from.as_str() {
                            pass = false;
                        }
                    }
                }

                if pass {
                    if let Some(ref date_to) = filters.date_to {
                        let stored = metadata
                            .and_then(|m| m.get("created_at"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if stored > date_to.as_str() {
                            pass = false;
                        }
                    }
                }

                if pass {
                    if let Some(ref filter_tags) = filters.tags {
                        let stored = metadata
                            .and_then(|m| m.get("tags"))
                            .and_then(|v| v.as_array());
                        let has_match = stored
                            .map(|arr| {
                                filter_tags.iter().any(|tag| {
                                    arr.iter().any(|v| v.as_str() == Some(tag.as_str()))
                                })
                            })
                            .unwrap_or(false);
                        if !has_match {
                            pass = false;
                        }
                    }
                }

                pass
            }
            None => false,
        };

        if !matches {
            result_set.remove(&id);
        }
    }

    Ok(Some(result_set))
}

/// Parse entity filters from a natural language query string.
///
/// Detects patterns like:
/// - "invoices over $500" -> min_amount = 500.0
/// - "invoices under $1000" -> max_amount = 1000.0
/// - "documents before 2024-01-01" -> before_date = "2024-01-01"
/// - "documents after 2023-06-01" -> after_date = "2023-06-01"
pub fn parse_entity_filter(query: &str) -> Option<EntityFilter> {
    let lower = query.to_lowercase();

    // Amount patterns
    let over_re = Regex::new(r"(?:over|above|more\s+than|greater\s+than|exceeding)\s+\$?([\d,]+(?:\.\d{2})?)").unwrap();
    let under_re = Regex::new(r"(?:under|below|less\s+than|cheaper\s+than)\s+\$?([\d,]+(?:\.\d{2})?)").unwrap();

    // Date patterns
    let before_re = Regex::new(r"before\s+(\d{4}-\d{2}-\d{2})").unwrap();
    let after_re = Regex::new(r"after\s+(\d{4}-\d{2}-\d{2})").unwrap();

    let min_amount = over_re.captures(&lower).and_then(|c| {
        c.get(1)
            .and_then(|m| m.as_str().replace(',', "").parse::<f64>().ok())
    });

    let max_amount = under_re.captures(&lower).and_then(|c| {
        c.get(1)
            .and_then(|m| m.as_str().replace(',', "").parse::<f64>().ok())
    });

    let before_date = before_re
        .captures(&lower)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));

    let after_date = after_re
        .captures(&lower)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));

    if min_amount.is_none() && max_amount.is_none() && before_date.is_none() && after_date.is_none()
    {
        return None;
    }

    Some(EntityFilter {
        min_amount,
        max_amount,
        before_date,
        after_date,
    })
}

/// Apply entity filter against a document's extracted_entities metadata.
///
/// Returns true if the document passes the filter (or if no relevant entities exist).
pub fn apply_entity_filter(
    entity_filter: &EntityFilter,
    metadata: &std::collections::HashMap<String, serde_json::Value>,
) -> bool {
    let entities = match metadata.get("extracted_entities") {
        Some(v) => match v.as_array() {
            Some(arr) => arr,
            None => return true,
        },
        None => return true,
    };

    // Check amount filters
    if entity_filter.min_amount.is_some() || entity_filter.max_amount.is_some() {
        let amounts: Vec<f64> = entities
            .iter()
            .filter(|e| {
                e.get("entity_type")
                    .and_then(|v| v.as_str())
                    == Some("amount")
            })
            .filter_map(|e| {
                e.get("value")
                    .and_then(|v| v.as_str())
                    .and_then(|s| {
                        let cleaned: String = s.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
                        cleaned.parse::<f64>().ok()
                    })
            })
            .collect();

        if !amounts.is_empty() {
            if let Some(min) = entity_filter.min_amount {
                if !amounts.iter().any(|a| *a >= min) {
                    return false;
                }
            }
            if let Some(max) = entity_filter.max_amount {
                if !amounts.iter().any(|a| *a <= max) {
                    return false;
                }
            }
        }
    }

    // Check date filters
    if entity_filter.before_date.is_some() || entity_filter.after_date.is_some() {
        let dates: Vec<&str> = entities
            .iter()
            .filter(|e| {
                e.get("entity_type")
                    .and_then(|v| v.as_str())
                    == Some("date")
            })
            .filter_map(|e| e.get("value").and_then(|v| v.as_str()))
            .collect();

        if !dates.is_empty() {
            if let Some(ref before) = entity_filter.before_date {
                if !dates.iter().any(|d| *d < before.as_str()) {
                    return false;
                }
            }
            if let Some(ref after) = entity_filter.after_date {
                if !dates.iter().any(|d| *d > after.as_str()) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_metadata_filters_all_none() {
        let filters = SearchFilters {
            doc_type: None,
            space_id: None,
            date_from: None,
            date_to: None,
            tags: None,
        };
        let tmp = std::env::temp_dir().join("cortex-test-filters-none");
        let _ = std::fs::remove_dir_all(&tmp);
        let engine = CortexEngine::new_with_path(tmp.clone()).unwrap();
        let result = apply_metadata_filters(&filters, &engine).unwrap();
        assert!(result.is_none(), "all-None filters should return None");
        let _ = std::fs::remove_dir_all(tmp);
    }

    #[test]
    fn test_parse_entity_filter_amount_over() {
        let filter = parse_entity_filter("invoices over $500").unwrap();
        assert_eq!(filter.min_amount, Some(500.0));
        assert!(filter.max_amount.is_none());
    }

    #[test]
    fn test_parse_entity_filter_amount_under() {
        let filter = parse_entity_filter("receipts under $1,000.00").unwrap();
        assert_eq!(filter.max_amount, Some(1000.0));
        assert!(filter.min_amount.is_none());
    }

    #[test]
    fn test_parse_entity_filter_date_before() {
        let filter = parse_entity_filter("documents before 2024-01-01").unwrap();
        assert_eq!(filter.before_date, Some("2024-01-01".to_string()));
    }

    #[test]
    fn test_parse_entity_filter_date_after() {
        let filter = parse_entity_filter("files after 2023-06-01").unwrap();
        assert_eq!(filter.after_date, Some("2023-06-01".to_string()));
    }

    #[test]
    fn test_parse_entity_filter_no_match() {
        let result = parse_entity_filter("find my tax documents");
        assert!(result.is_none());
    }

    #[test]
    fn test_apply_entity_filter_amount_pass() {
        let filter = EntityFilter {
            min_amount: Some(100.0),
            max_amount: None,
            before_date: None,
            after_date: None,
        };
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(
            "extracted_entities".to_string(),
            serde_json::json!([
                {"entity_type": "amount", "value": "$500.00", "label": "Amount"}
            ]),
        );
        assert!(apply_entity_filter(&filter, &metadata));
    }

    #[test]
    fn test_apply_entity_filter_amount_fail() {
        let filter = EntityFilter {
            min_amount: Some(1000.0),
            max_amount: None,
            before_date: None,
            after_date: None,
        };
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(
            "extracted_entities".to_string(),
            serde_json::json!([
                {"entity_type": "amount", "value": "$50.00", "label": "Amount"}
            ]),
        );
        assert!(!apply_entity_filter(&filter, &metadata));
    }

    #[test]
    fn test_apply_entity_filter_no_entities() {
        let filter = EntityFilter {
            min_amount: Some(100.0),
            max_amount: None,
            before_date: None,
            after_date: None,
        };
        let metadata = std::collections::HashMap::new();
        assert!(apply_entity_filter(&filter, &metadata));
    }
}
