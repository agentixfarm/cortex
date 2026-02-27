use tauri::State;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<Settings, AppError> {
    let _engine = state.engine.clone();
    let result = tokio::task::spawn_blocking(move || {
        // Phase 2 will load persisted settings from disk
        Ok::<Settings, AppError>(Settings {
            theme: "dark".to_string(),
            sidebar_collapsed: false,
            embedding_model: "local".to_string(),
            watched_folders: vec![],
            excluded_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                ".DS_Store".to_string(),
            ],
            index_on_startup: true,
            index_size: 0,
            storage_path: "~/Library/Application Support/com.cortex.app/vectors".to_string(),
        })
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn update_settings(
    settings: Settings,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let _engine = state.engine.clone();
    let _settings = settings;
    tokio::task::spawn_blocking(move || {
        // Phase 2 will persist settings to disk and apply changes
        Ok::<(), AppError>(())
    })
    .await??;
    Ok(())
}
