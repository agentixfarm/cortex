use std::collections::HashMap;

/// Name a space based on the metadata of its member documents.
///
/// Returns (name, icon, color) using rule-based heuristics:
/// - Dominant entity types (amount -> Invoices, person -> Contacts)
/// - Dominant document types (xlsx -> Spreadsheets)
/// - Path segment patterns (work, medical, kids, property)
/// - Fallback: "Space {N}"
pub fn name_space(doc_metadata: &[HashMap<String, serde_json::Value>], space_index: usize) -> (String, String, String) {
    let mut entity_counts: HashMap<String, usize> = HashMap::new();
    let mut doc_type_counts: HashMap<String, usize> = HashMap::new();
    let mut path_segments: HashMap<String, usize> = HashMap::new();

    for meta in doc_metadata {
        // Count entity types
        if let Some(entities) = meta.get("extracted_entities").and_then(|v| v.as_array()) {
            for entity in entities {
                if let Some(etype) = entity.get("entity_type").and_then(|v| v.as_str()) {
                    *entity_counts.entry(etype.to_lowercase()).or_insert(0) += 1;
                }
            }
        }

        // Count doc types
        if let Some(doc_type) = meta.get("doc_type").and_then(|v| v.as_str()) {
            *doc_type_counts.entry(doc_type.to_lowercase()).or_insert(0) += 1;
        }

        // Collect path segments
        if let Some(path) = meta.get("path").and_then(|v| v.as_str()) {
            for segment in path.to_lowercase().split('/') {
                let seg = segment.trim();
                if !seg.is_empty() && seg.len() > 2 {
                    *path_segments.entry(seg.to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    let dominant_entity = entity_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(k, _)| k.as_str());

    let dominant_doc_type = doc_type_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(k, _)| k.as_str());

    // Check path patterns
    let has_path = |pattern: &str| -> bool {
        path_segments.keys().any(|seg| seg.contains(pattern))
    };

    // Rule-based naming heuristics
    if dominant_entity == Some("amount") && dominant_doc_type == Some("pdf") {
        return (
            "Invoices & Receipts".to_string(),
            "Receipt".to_string(),
            "#10B981".to_string(),
        );
    }

    if has_path("tax") || has_path("property") {
        if dominant_entity == Some("date") || dominant_entity == Some("amount") {
            return (
                "Property".to_string(),
                "Home".to_string(),
                "#6366F1".to_string(),
            );
        }
    }

    if dominant_entity == Some("person") || dominant_entity == Some("organization") {
        return (
            "Contacts & Correspondence".to_string(),
            "Users".to_string(),
            "#F59E0B".to_string(),
        );
    }

    if dominant_doc_type == Some("xlsx") || dominant_doc_type == Some("csv") {
        return (
            "Spreadsheets & Data".to_string(),
            "Table".to_string(),
            "#3B82F6".to_string(),
        );
    }

    if has_path("work") || has_path("project") {
        return (
            "Work".to_string(),
            "Briefcase".to_string(),
            "#8B5CF6".to_string(),
        );
    }

    if has_path("medical") || has_path("health") {
        return (
            "Medical".to_string(),
            "Heart".to_string(),
            "#EF4444".to_string(),
        );
    }

    if has_path("kid") || has_path("school") || has_path("education") {
        return (
            "Kids & Education".to_string(),
            "GraduationCap".to_string(),
            "#F97316".to_string(),
        );
    }

    if has_path("photo") || has_path("image") || has_path("picture") {
        if dominant_doc_type == Some("png") || dominant_doc_type == Some("jpg") {
            return (
                "Photos & Images".to_string(),
                "Image".to_string(),
                "#EC4899".to_string(),
            );
        }
    }

    // Default fallback
    (
        format!("Space {}", space_index + 1),
        "Folder".to_string(),
        "#6B7280".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_name_space_invoices() {
        let meta = vec![
            {
                let mut m = HashMap::new();
                m.insert("doc_type".to_string(), json!("pdf"));
                m.insert(
                    "extracted_entities".to_string(),
                    json!([
                        {"entity_type": "amount", "value": "$500", "label": "Amount"},
                        {"entity_type": "amount", "value": "$200", "label": "Amount"},
                    ]),
                );
                m.insert("path".to_string(), json!("/invoices/receipt.pdf"));
                m
            },
            {
                let mut m = HashMap::new();
                m.insert("doc_type".to_string(), json!("pdf"));
                m.insert(
                    "extracted_entities".to_string(),
                    json!([
                        {"entity_type": "amount", "value": "$100", "label": "Amount"},
                    ]),
                );
                m.insert("path".to_string(), json!("/invoices/bill.pdf"));
                m
            },
        ];

        let (name, icon, color) = name_space(&meta, 0);
        assert_eq!(name, "Invoices & Receipts");
        assert_eq!(icon, "Receipt");
        assert_eq!(color, "#10B981");
    }

    #[test]
    fn test_name_space_work() {
        let meta = vec![{
            let mut m = HashMap::new();
            m.insert("doc_type".to_string(), json!("docx"));
            m.insert("path".to_string(), json!("/work/reports/q1.docx"));
            m
        }];

        let (name, icon, _) = name_space(&meta, 0);
        assert_eq!(name, "Work");
        assert_eq!(icon, "Briefcase");
    }

    #[test]
    fn test_name_space_default_fallback() {
        let meta = vec![{
            let mut m = HashMap::new();
            m.insert("doc_type".to_string(), json!("other"));
            m.insert("path".to_string(), json!("/random/stuff.bin"));
            m
        }];

        let (name, icon, _) = name_space(&meta, 2);
        assert_eq!(name, "Space 3");
        assert_eq!(icon, "Folder");
    }

    #[test]
    fn test_name_space_medical() {
        let meta = vec![{
            let mut m = HashMap::new();
            m.insert("doc_type".to_string(), json!("pdf"));
            m.insert("path".to_string(), json!("/medical/records/report.pdf"));
            m
        }];

        let (name, _, _) = name_space(&meta, 0);
        assert_eq!(name, "Medical");
    }

    #[test]
    fn test_name_space_empty_metadata() {
        let meta: Vec<HashMap<String, serde_json::Value>> = vec![];
        let (name, icon, _) = name_space(&meta, 0);
        assert_eq!(name, "Space 1");
        assert_eq!(icon, "Folder");
    }
}
