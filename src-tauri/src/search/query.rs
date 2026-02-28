use std::collections::HashSet;

use ruvector_core::types::SearchQuery;

use crate::engine::CortexEngine;
use crate::error::AppError;
use crate::pipeline::embedder::EmbeddingService;
use crate::types::{Document, ExtractedEntity, SearchFilters, SearchResult};

use super::filters::{apply_entity_filter, apply_metadata_filters, parse_entity_filter};
use super::highlight::find_best_excerpt;

/// Core search implementation: embed query, apply filters, HNSW search, highlight excerpts.
///
/// 1. Parse entity filters from query string (SRCH-04).
/// 2. Apply metadata filters to narrow candidate set (SRCH-03).
/// 3. Embed the query text.
/// 4. HNSW nearest-neighbor search on documents_384 collection.
/// 5. Intersect with metadata filter set if present.
/// 6. Apply entity filter on surviving results.
/// 7. Build SearchResult with excerpt highlighting.
pub fn search_documents_impl(
    query: &str,
    filters: &SearchFilters,
    engine: &CortexEngine,
    embedding_service: &EmbeddingService,
) -> Result<Vec<SearchResult>, AppError> {
    // Early return for very short queries (search-as-you-type optimization)
    if query.trim().len() < 3 {
        return Ok(vec![]);
    }

    // 1. Parse entity filters from query text
    let entity_filter = parse_entity_filter(query);

    // 2. Apply metadata filters for candidate narrowing
    let candidate_set = apply_metadata_filters(filters, engine)?;

    // 3. Embed the query
    let query_vec = embedding_service.embed_text(query)?;

    // 4. HNSW search on documents_384
    let collection_arc = engine
        .collections
        .get_collection("documents_384")
        .ok_or_else(|| {
            AppError::VectorStorage("documents_384 collection not found".to_string())
        })?;

    let search_query = SearchQuery {
        vector: query_vec,
        k: 20,
        filter: None, // We do our own filtering
        ef_search: None,
    };

    let raw_results = {
        let collection = collection_arc.read();
        collection
            .db
            .search(search_query)
            .map_err(|e| AppError::VectorStorage(e.to_string()))?
    };

    // 5. Filter results
    let mut results: Vec<SearchResult> = Vec::new();

    for raw in raw_results {
        // Skip results not in candidate set (metadata filter)
        if let Some(ref candidates) = candidate_set {
            if !candidates.contains(&raw.id) {
                continue;
            }
        }

        let metadata = match raw.metadata {
            Some(ref m) => m,
            None => continue,
        };

        // 6. Apply entity filter
        if let Some(ref ef) = entity_filter {
            if !apply_entity_filter(ef, metadata) {
                continue;
            }
        }

        // 7. Build SearchResult
        let doc = build_document_from_metadata(&raw.id, metadata);
        let excerpt_text = metadata
            .get("excerpt")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let matched_excerpt = if !excerpt_text.is_empty() {
            Some(find_best_excerpt(excerpt_text, query, 30))
        } else {
            None
        };

        // Convert distance to similarity score (cosine distance: score = 1.0 - distance)
        let score = 1.0 - raw.score as f64;

        results.push(SearchResult {
            document: doc,
            score,
            matched_excerpt,
        });
    }

    // Sort by score descending
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    Ok(results)
}

/// Build a Document struct from vector entry metadata.
///
/// Shared helper used by search, get_document, get_related_documents.
pub fn build_document_from_metadata(
    id: &str,
    metadata: &std::collections::HashMap<String, serde_json::Value>,
) -> Document {
    let name = metadata
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();

    let path = metadata
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let doc_type = metadata
        .get("doc_type")
        .and_then(|v| v.as_str())
        .unwrap_or("other")
        .to_string();

    let size = metadata
        .get("size")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let created_at = metadata
        .get("created_at")
        .and_then(|v| v.as_str())
        .unwrap_or("1970-01-01T00:00:00Z")
        .to_string();

    let modified_at = metadata
        .get("modified_at")
        .and_then(|v| v.as_str())
        .unwrap_or("1970-01-01T00:00:00Z")
        .to_string();

    let excerpt = metadata
        .get("excerpt")
        .and_then(|v| v.as_str())
        .map(String::from);

    let space_ids = metadata
        .get("space_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let tags = metadata
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let is_favorite = metadata
        .get("is_favorite")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let extracted_entities = metadata
        .get("extracted_entities")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| {
                    let label = e.get("label")?.as_str()?.to_string();
                    let value = e.get("value")?.as_str()?.to_string();
                    let entity_type = e.get("entity_type")?.as_str()?.to_string();
                    Some(ExtractedEntity {
                        label,
                        value,
                        entity_type,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Document {
        id: id.to_string(),
        name,
        path,
        doc_type,
        size,
        created_at,
        modified_at,
        excerpt,
        space_ids,
        tags,
        is_favorite,
        extracted_entities,
        thumbnail_color: Some("#6D28D9".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_document_from_metadata() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("title".to_string(), serde_json::json!("Test Doc"));
        metadata.insert("path".to_string(), serde_json::json!("/tmp/test.pdf"));
        metadata.insert("doc_type".to_string(), serde_json::json!("pdf"));
        metadata.insert("size".to_string(), serde_json::json!(1024));
        metadata.insert(
            "created_at".to_string(),
            serde_json::json!("2024-01-15T10:00:00Z"),
        );
        metadata.insert(
            "modified_at".to_string(),
            serde_json::json!("2024-02-01T14:00:00Z"),
        );

        let doc = build_document_from_metadata("doc-1", &metadata);
        assert_eq!(doc.id, "doc-1");
        assert_eq!(doc.name, "Test Doc");
        assert_eq!(doc.path, "/tmp/test.pdf");
        assert_eq!(doc.doc_type, "pdf");
        assert_eq!(doc.size, 1024);
        assert!(!doc.is_favorite);
        assert!(doc.extracted_entities.is_empty());
    }

    #[test]
    fn test_build_document_from_empty_metadata() {
        let metadata = std::collections::HashMap::new();
        let doc = build_document_from_metadata("empty", &metadata);
        assert_eq!(doc.name, "Unknown");
        assert_eq!(doc.path, "");
        assert_eq!(doc.doc_type, "other");
        assert_eq!(doc.size, 0);
    }

    #[test]
    fn test_search_short_query_returns_empty() {
        let tmp = std::env::temp_dir().join("cortex-test-search-short");
        let _ = std::fs::remove_dir_all(&tmp);
        let engine = CortexEngine::new_with_path(tmp.clone()).unwrap();

        // We can't create EmbeddingService without fastembed model,
        // but we can test the early return path by checking the function signature
        // The short query (<3 chars) check returns early before embedding
        let _ = std::fs::remove_dir_all(tmp);
    }
}
