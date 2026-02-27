use tauri::State;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn add_watched_folder(
    path: String,
    state: State<'_, AppState>,
) -> Result<WatchedFolder, AppError> {
    let _engine = state.engine.clone();
    let result = tokio::task::spawn_blocking(move || {
        // Phase 2 will register the folder with notify-rs file watcher
        Ok::<WatchedFolder, AppError>(WatchedFolder {
            id: format!("folder-{:x}", path.len()),
            path: path.clone(),
            document_count: 0,
            last_scan: "2026-02-27T00:00:00Z".to_string(),
            status: "watching".to_string(),
        })
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn remove_watched_folder(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let _engine = state.engine.clone();
    let _id = id;
    tokio::task::spawn_blocking(move || {
        // Phase 2 will stop watching and remove the folder from the watcher registry
        Ok::<(), AppError>(())
    })
    .await??;
    Ok(())
}

#[tauri::command]
pub async fn trigger_scan(
    folder_id: String,
    state: State<'_, AppState>,
) -> Result<ScanProgress, AppError> {
    let _engine = state.engine.clone();
    let result = tokio::task::spawn_blocking(move || {
        // Phase 2 will initiate a real document scan via the indexing pipeline
        Ok::<ScanProgress, AppError>(ScanProgress {
            folder_id: folder_id.clone(),
            total_files: 0,
            processed_files: 0,
            status: "scanning".to_string(),
        })
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn get_watched_folders(
    state: State<'_, AppState>,
) -> Result<Vec<WatchedFolder>, AppError> {
    let _engine = state.engine.clone();
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will return persisted watched folders from the watcher registry
        Ok::<Vec<WatchedFolder>, AppError>(vec![])
    })
    .await??;
    Ok(results)
}
