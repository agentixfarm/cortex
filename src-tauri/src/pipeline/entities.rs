use regex::Regex;
use crate::types::ExtractedEntity;

/// Regex-based entity extractor for dates, amounts, emails, and person/org names.
pub struct EntityExtractor {
    date_re: Regex,
    amount_re: Regex,
    email_re: Regex,
    person_re: Regex,
}

impl EntityExtractor {
    /// Construct a new EntityExtractor, compiling all regex patterns once.
    pub fn new() -> Self {
        let date_re = Regex::new(
            r"\b(\d{4}-\d{2}-\d{2}|\d{1,2}/\d{1,2}/\d{2,4}|(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+\d{1,2},?\s+\d{4})\b"
        ).expect("date regex is valid");

        let amount_re = Regex::new(
            r"(?:USD\s*)?[$\u{00a3}\u{20ac}]\s*[\d,]+(?:\.\d{2})?|[\d,]+(?:\.\d{2})?\s*(?:USD|EUR|GBP)"
        ).expect("amount regex is valid");

        let email_re = Regex::new(
            r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b"
        ).expect("email regex is valid");

        let person_re = Regex::new(
            r"\b([A-Z][a-z]+\s+[A-Z][a-z]+)\b"
        ).expect("person regex is valid");

        Self { date_re, amount_re, email_re, person_re }
    }

    /// Extract entities from text. Results are deduplicated and capped at 20.
    pub fn extract(&self, text: &str) -> Vec<ExtractedEntity> {
        let mut entities: Vec<ExtractedEntity> = Vec::new();

        for cap in self.date_re.captures_iter(text) {
            entities.push(ExtractedEntity {
                label: "Date".to_string(),
                value: cap[0].to_string(),
                entity_type: "date".to_string(),
            });
        }

        for cap in self.amount_re.find_iter(text) {
            entities.push(ExtractedEntity {
                label: "Amount".to_string(),
                value: cap.as_str().to_string(),
                entity_type: "amount".to_string(),
            });
        }

        for cap in self.email_re.find_iter(text) {
            entities.push(ExtractedEntity {
                label: "Email".to_string(),
                value: cap.as_str().to_string(),
                entity_type: "person".to_string(),
            });
        }

        for cap in self.person_re.captures_iter(text) {
            entities.push(ExtractedEntity {
                label: "Person/Org".to_string(),
                value: cap[1].to_string(),
                entity_type: "person".to_string(),
            });
        }

        // Deduplicate by value
        entities.sort_by(|a, b| a.value.cmp(&b.value));
        entities.dedup_by(|a, b| a.value == b.value);

        // Cap at 20 entities
        entities.truncate(20);

        entities
    }
}

impl Default for EntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extractor() -> EntityExtractor {
        EntityExtractor::new()
    }

    #[test]
    fn test_extract_iso_date() {
        let e = extractor();
        let results = e.extract("Meeting on 2024-03-15 to discuss Q2.");
        let dates: Vec<_> = results.iter().filter(|e| e.entity_type == "date").collect();
        assert!(!dates.is_empty(), "should find at least one date");
        assert!(dates.iter().any(|d| d.value == "2024-03-15"));
    }

    #[test]
    fn test_extract_us_date() {
        let e = extractor();
        let results = e.extract("Due 3/15/2024 please submit.");
        let dates: Vec<_> = results.iter().filter(|e| e.entity_type == "date").collect();
        assert!(!dates.is_empty());
        assert!(dates.iter().any(|d| d.value == "3/15/2024"));
    }

    #[test]
    fn test_extract_written_date() {
        let e = extractor();
        let results = e.extract("Invoice dated January 15, 2024.");
        let dates: Vec<_> = results.iter().filter(|e| e.entity_type == "date").collect();
        assert!(!dates.is_empty(), "should find written date");
    }

    #[test]
    fn test_extract_dollar_amount() {
        let e = extractor();
        let results = e.extract("Total: $1,234.56 due by end of month.");
        let amounts: Vec<_> = results.iter().filter(|e| e.entity_type == "amount").collect();
        assert!(!amounts.is_empty());
        assert!(amounts.iter().any(|a| a.value.contains("1,234.56")));
    }

    #[test]
    fn test_extract_person_name() {
        let e = extractor();
        // Use text where John Smith is not preceded by another capitalized word
        let results = e.extract("Please reach out to John Smith regarding the invoice.");
        let persons: Vec<_> = results.iter().filter(|e| e.entity_type == "person").collect();
        assert!(!persons.is_empty());
        assert!(persons.iter().any(|p| p.value == "John Smith"), "got: {:?}", persons);
    }

    #[test]
    fn test_deduplication() {
        let e = extractor();
        let results = e.extract("John Smith contacted John Smith again.");
        let persons: Vec<_> = results.iter().filter(|p| p.value == "John Smith").collect();
        assert_eq!(persons.len(), 1, "duplicate values should be removed");
    }

    #[test]
    fn test_truncation_at_20() {
        let e = extractor();
        // Generate text with many distinct amounts
        let text: String = (1..=30)
            .map(|i| format!("${}.00 ", i * 100))
            .collect();
        let results = e.extract(&text);
        assert!(results.len() <= 20, "should cap at 20 entities, got {}", results.len());
    }

    #[test]
    fn test_empty_text_returns_empty() {
        let e = extractor();
        let results = e.extract("");
        assert!(results.is_empty());
    }

    #[test]
    fn test_no_false_positives_on_plain_text() {
        let e = extractor();
        let results = e.extract("the quick brown fox jumps over the lazy dog");
        // No dates, amounts, or emails in this text
        let dates: Vec<_> = results.iter().filter(|e| e.entity_type == "date").collect();
        let amounts: Vec<_> = results.iter().filter(|e| e.entity_type == "amount").collect();
        assert!(dates.is_empty());
        assert!(amounts.is_empty());
    }
}
