use tauri::State;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn get_spaces(
    state: State<'_, AppState>,
) -> Result<Vec<Space>, AppError> {
    let _engine = state.engine.clone();
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will return real spaces from RuVector GNN clustering
        Ok::<Vec<Space>, AppError>(vec![])
    })
    .await??;
    Ok(results)
}

#[tauri::command]
pub async fn get_space_documents(
    space_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Document>, AppError> {
    let _engine = state.engine.clone();
    let _space_id = space_id;
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will query RuVector collections for documents in this space
        Ok::<Vec<Document>, AppError>(vec![])
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
    let _engine = state.engine.clone();
    let _doc_id = doc_id;
    let _space_id = space_id;
    tokio::task::spawn_blocking(move || {
        // Phase 2 will update the document's space assignment in RuVector
        Ok::<(), AppError>(())
    })
    .await??;
    Ok(())
}

#[tauri::command]
pub async fn recluster_spaces(
    state: State<'_, AppState>,
) -> Result<Vec<Space>, AppError> {
    let _engine = state.engine.clone();
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will trigger a full GNN recluster via ruvector-gnn
        Ok::<Vec<Space>, AppError>(vec![])
    })
    .await??;
    Ok(results)
}
