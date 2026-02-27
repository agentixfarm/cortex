use tauri::State;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn index_document(
    path: String,
    state: State<'_, AppState>,
) -> Result<DocumentMeta, AppError> {
    let _engine = state.engine.clone();
    let result = tokio::task::spawn_blocking(move || {
        // Phase 2 will implement real document indexing via the pipeline
        Ok::<DocumentMeta, AppError>(DocumentMeta {
            id: format!("doc-{}", uuid_stub(&path)),
            name: path.split('/').last().unwrap_or("unknown").to_string(),
            path: path.clone(),
            doc_type: detect_doc_type(&path),
            size: 0,
        })
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn search_documents(
    query: String,
    filters: SearchFilters,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, AppError> {
    let _engine = state.engine.clone();
    let _query = query;
    let _filters = filters;
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will implement real search via RuVector
        Ok::<Vec<SearchResult>, AppError>(vec![])
    })
    .await??;
    Ok(results)
}

#[tauri::command]
pub async fn get_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<Document, AppError> {
    let _engine = state.engine.clone();
    let result = tokio::task::spawn_blocking(move || {
        // Phase 2 will look up the real document from RuVector
        Ok::<Document, AppError>(Document {
            id: id.clone(),
            name: "Sample Document.pdf".to_string(),
            path: format!("~/Documents/{}.pdf", id),
            doc_type: "pdf".to_string(),
            size: 204800,
            created_at: "2026-01-15T10:30:00Z".to_string(),
            modified_at: "2026-02-01T14:22:00Z".to_string(),
            excerpt: Some("This is a sample document excerpt for stub data.".to_string()),
            space_ids: vec!["space-work".to_string()],
            tags: vec!["document".to_string(), "sample".to_string()],
            is_favorite: false,
            extracted_entities: vec![
                ExtractedEntity {
                    label: "Date".to_string(),
                    value: "2026-01-15".to_string(),
                    entity_type: "date".to_string(),
                },
            ],
            thumbnail_color: Some("#6D28D9".to_string()),
        })
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn get_related_documents(
    id: String,
    limit: usize,
    state: State<'_, AppState>,
) -> Result<Vec<Document>, AppError> {
    let _engine = state.engine.clone();
    let _id = id;
    let _limit = limit;
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will use RuVector graph queries to find related documents
        Ok::<Vec<Document>, AppError>(vec![])
    })
    .await??;
    Ok(results)
}

#[tauri::command]
pub async fn toggle_favorite(
    id: String,
    state: State<'_, AppState>,
) -> Result<bool, AppError> {
    let _engine = state.engine.clone();
    let _id = id;
    let result = tokio::task::spawn_blocking(move || {
        // Phase 2 will persist the favorite flag in RuVector metadata
        Ok::<bool, AppError>(true)
    })
    .await??;
    Ok(result)
}

// --- Helpers (stub only, not exported) ---

fn uuid_stub(input: &str) -> String {
    // Simple deterministic stub ID from input — Phase 2 uses real UUIDs
    format!("{:x}", input.len())
}

fn detect_doc_type(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "pdf" => "pdf".to_string(),
        "docx" | "doc" => "docx".to_string(),
        "txt" => "txt".to_string(),
        "png" => "png".to_string(),
        "jpg" | "jpeg" => "jpg".to_string(),
        "xlsx" | "xls" => "xlsx".to_string(),
        "csv" => "csv".to_string(),
        "md" => "md".to_string(),
        _ => "other".to_string(),
    }
}
