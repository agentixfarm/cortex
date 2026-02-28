use tauri::State;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn index_document(
    path: String,
    state: State<'_, AppState>,
) -> Result<DocumentMeta, AppError> {
    let engine = state.engine.clone();
    let embedding_service = state.embedding_service.clone();
    let indexer = state.indexer.clone();
    let path_owned = path.clone();

    let doc_id = tokio::task::spawn_blocking(move || {
        let file_path = std::path::Path::new(&path_owned);
        let engine_guard = engine.blocking_lock();
        indexer.index_file(file_path, &engine_guard, &embedding_service)
    })
    .await??;

    let file_path = std::path::Path::new(&path);
    let name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    let doc_type = detect_doc_type(file_path.to_str().unwrap_or(""));
    let size = std::fs::metadata(&path).ok().map(|m| m.len()).unwrap_or(0);
    Ok(DocumentMeta {
        id: doc_id,
        name,
        path,
        doc_type,
        size,
    })
}

#[tauri::command]
pub async fn search_documents(
    query: String,
    filters: SearchFilters,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, AppError> {
    let engine = state.engine.clone();
    let embedding_service = state.embedding_service.clone();
    let query_owned = query.clone();
    let filters_owned = filters.clone();

    let results = tokio::task::spawn_blocking(move || {
        let engine_guard = engine.blocking_lock();
        crate::search::query::search_documents_impl(
            &query_owned,
            &filters_owned,
            &engine_guard,
            &embedding_service,
        )
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
        // Phase 3 will look up the real document from RuVector
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
    let graph = state.doc_graph.clone();
    let engine = state.engine.clone();
    let id_owned = id;

    let results = tokio::task::spawn_blocking(move || {
        let graph_guard = graph
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let engine_guard = engine.blocking_lock();
        crate::graph::related::get_related_impl(&id_owned, limit, &graph_guard, &engine_guard)
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
        // Phase 4 will persist the favorite flag in RuVector metadata
        Ok::<bool, AppError>(true)
    })
    .await??;
    Ok(result)
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
