use tauri::State;
use crate::error::AppError;
use crate::search::query::build_document_from_metadata;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn get_spaces(
    state: State<'_, AppState>,
) -> Result<Vec<Space>, AppError> {
    let space_mgr = state.space_manager.clone();
    let results = tokio::task::spawn_blocking(move || {
        let guard = space_mgr
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok::<Vec<Space>, AppError>(guard.get_spaces())
    })
    .await??;
    Ok(results)
}

#[tauri::command]
pub async fn get_space_documents(
    space_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Document>, AppError> {
    let space_mgr = state.space_manager.clone();
    let engine = state.engine.clone();

    let results = tokio::task::spawn_blocking(move || {
        let space_guard = space_mgr
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let doc_ids = space_guard.get_space_documents(&space_id);

        let engine_guard = engine.blocking_lock();
        let collection_arc = engine_guard
            .collections
            .get_collection("documents_384")
            .ok_or_else(|| {
                AppError::VectorStorage("documents_384 collection not found".to_string())
            })?;
        let collection = collection_arc.read();

        let mut documents: Vec<Document> = Vec::new();
        for id in doc_ids {
            let entry = collection
                .db
                .get(&id)
                .map_err(|e| AppError::VectorStorage(e.to_string()))?;
            if let Some(entry) = entry {
                if let Some(ref metadata) = entry.metadata {
                    documents.push(build_document_from_metadata(&id, metadata));
                }
            }
        }

        Ok::<Vec<Document>, AppError>(documents)
    })
    .await??;
    Ok(results)
}

#[tauri::command]
pub async fn move_document_to_space(
    doc_id: String,
    space_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let space_mgr = state.space_manager.clone();

    tokio::task::spawn_blocking(move || {
        let mut guard = space_mgr
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        guard.move_document(&doc_id, &space_id)
    })
    .await??;
    Ok(())
}

#[tauri::command]
pub async fn recluster_spaces(
    state: State<'_, AppState>,
) -> Result<Vec<Space>, AppError> {
    let engine = state.engine.clone();
    let space_mgr = state.space_manager.clone();
    let doc_graph = state.doc_graph.clone();

    let results = tokio::task::spawn_blocking(move || {
        let engine_guard = engine.blocking_lock();
        let mut space_guard = space_mgr
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let spaces = space_guard.recluster(&engine_guard)?;

        // Rebuild document graph after recluster
        let mut graph_guard = doc_graph
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        graph_guard.build_edges(&engine_guard, &space_guard)?;

        Ok::<Vec<Space>, AppError>(spaces)
    })
    .await??;
    Ok(results)
}
