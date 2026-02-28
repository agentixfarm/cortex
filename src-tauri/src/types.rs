use serde::{Deserialize, Serialize};

// === Document types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub name: String,
    pub path: String,
    pub doc_type: String,  // "pdf", "docx", "txt", etc.
    pub size: u64,
    pub created_at: String,  // ISO 8601
    pub modified_at: String,
    pub excerpt: Option<String>,
    pub space_ids: Vec<String>,
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub extracted_entities: Vec<ExtractedEntity>,
    pub thumbnail_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedEntity {
    pub label: String,
    pub value: String,
    pub entity_type: String,  // "date", "amount", "person", "organization", "location"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMeta {
    pub id: String,
    pub name: String,
    pub path: String,
    pub doc_type: String,
    pub size: u64,
    pub created_at: String,  // ISO 8601
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilters {
    pub doc_type: Option<String>,
    pub space_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub document: Document,
    pub score: f64,
    pub matched_excerpt: Option<String>,
}

// === Space types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub document_count: u32,
    pub last_updated: String,
    pub sub_spaces: Vec<Space>,
    pub parent_id: Option<String>,
    pub sample_files: Vec<String>,
}

// === Folder types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchedFolder {
    pub id: String,
    pub path: String,
    pub document_count: u32,
    pub last_scan: String,
    pub status: String,  // "watching", "paused", "error"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgress {
    pub folder_id: String,
    pub total_files: u32,
    pub processed_files: u32,
    pub status: String,  // "scanning", "complete", "error"
}

// === Analytics types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_documents: u32,
    pub smart_spaces: u32,
    pub last_scan: String,
    pub index_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceGraphNode {
    pub id: String,
    pub name: String,
    pub color: String,
    pub document_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceGraphEdge {
    pub source: String,
    pub target: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceGraph {
    pub nodes: Vec<SpaceGraphNode>,
    pub edges: Vec<SpaceGraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopQuery {
    pub query: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAnalytics {
    pub total_searches: u32,
    pub top_queries: Vec<TopQuery>,
    pub avg_results_per_query: f64,
    pub queries_this_week: u32,
}

// === Settings types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub theme: String,              // "dark", "light", "system"
    pub sidebar_collapsed: bool,
    pub embedding_model: String,    // "local", "openai"
    pub watched_folders: Vec<String>,
    pub excluded_patterns: Vec<String>,
    pub index_on_startup: bool,
    pub index_size: u64,            // Bytes -- visible in Settings > Storage
    pub storage_path: String,       // Path to RuVector data dir -- visible in Settings > Storage
}

// === Tag types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
    pub document_count: u32,
    pub tag_type: String,  // "auto", "user"
}

// === Activity types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItem {
    pub id: String,
    pub action: String,    // "indexed", "moved", "tagged", "searched"
    pub subject: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub activity_type: String,  // "info", "success", "warning", "error"
    pub document_id: Option<String>,
}
