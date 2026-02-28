use tauri::State;
use tauri::Emitter;
use crate::error::AppError;
use crate::state::{AppState, WatcherCommand};
use crate::types::*;
use crate::watcher::worker::IndexProgress;

#[tauri::command]
pub async fn add_watched_folder(
    path: String,
    state: State<'_, AppState>,
) -> Result<WatchedFolder, AppError> {
    let config = {
        let mut registry = state.registry.lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let config = registry.add_folder(path.clone());
        registry.save(&state.registry_path)?;
        config
    };

    // Notify watcher task to start watching
    state.watcher_tx.send(WatcherCommand::AddFolder {
        path,
        folder_id: config.id.clone(),
    }).await.map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(WatchedFolder {
        id: config.id,
        path: config.path,
        document_count: config.document_count,
        last_scan: config.last_scan.unwrap_or_else(|| "never".to_string()),
        status: if config.is_paused { "paused".to_string() } else { "watching".to_string() },
    })
}

#[tauri::command]
pub async fn remove_watched_folder(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let path = {
        let mut registry = state.registry.lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let folder_path = registry.folders.get(&id)
            .map(|c| c.path.clone())
            .ok_or_else(|| AppError::NotFound(format!("Folder {id} not found")))?;
        registry.remove_folder(&id);
        registry.save(&state.registry_path)?;
        folder_path
    };

    // Notify watcher task to stop watching
    state.watcher_tx.send(WatcherCommand::RemoveFolder {
        folder_id: id,
        path,
    }).await.map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub async fn trigger_scan(
    folder_id: String,
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<ScanProgress, AppError> {
    let folder_config = {
        let registry = state.registry.lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        registry.folders.get(&folder_id).cloned()
            .ok_or_else(|| AppError::NotFound(format!("Folder {folder_id} not found")))?
    };

    let engine = state.engine.clone();
    let embedding_service = state.embedding_service.clone();
    let indexer = state.indexer.clone();
    let registry = state.registry.clone();
    let fid = folder_id.clone();

    // Spawn background scan task — returns immediately, progress via events
    tauri::async_runtime::spawn(async move {
        let folder_path = std::path::Path::new(&folder_config.path);
        if let Ok(entries) = walk_dir_recursive(folder_path) {
            for file_path in entries {
                let ext = file_path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");

                // Check type enabled and exclusions
                let (excluded, type_ok) = {
                    let reg = registry.lock().unwrap();
                    (reg.is_excluded(&fid, &file_path), reg.is_type_enabled(&fid, ext))
                };
                if excluded || !type_ok {
                    continue;
                }

                let path_str = file_path.to_string_lossy().to_string();
                let _ = app_handle.emit("index-progress", IndexProgress {
                    file_path: path_str.clone(),
                    status: "indexing".to_string(),
                    doc_id: None,
                    error: None,
                    folder_id: Some(fid.clone()),
                });

                let eng = engine.clone();
                let emb = embedding_service.clone();
                let idx = indexer.clone();
                let fp = file_path.clone();

                let result = tokio::task::spawn_blocking(move || {
                    let engine_guard = eng.blocking_lock();
                    idx.index_file(&fp, &engine_guard, &emb)
                }).await;

                match result {
                    Ok(Ok(doc_id)) => {
                        let _ = app_handle.emit("index-progress", IndexProgress {
                            file_path: path_str,
                            status: "indexed".to_string(),
                            doc_id: Some(doc_id),
                            error: None,
                            folder_id: Some(fid.clone()),
                        });
                    }
                    Ok(Err(e)) => {
                        let _ = app_handle.emit("index-progress", IndexProgress {
                            file_path: path_str,
                            status: "error".to_string(),
                            doc_id: None,
                            error: Some(e.to_string()),
                            folder_id: Some(fid.clone()),
                        });
                    }
                    Err(e) => {
                        let _ = app_handle.emit("index-progress", IndexProgress {
                            file_path: path_str,
                            status: "error".to_string(),
                            doc_id: None,
                            error: Some(e.to_string()),
                            folder_id: Some(fid.clone()),
                        });
                    }
                }
            }
        }

        // Emit scan complete
        let _ = app_handle.emit("index-progress", IndexProgress {
            file_path: folder_config.path,
            status: "scan-complete".to_string(),
            doc_id: None,
            error: None,
            folder_id: Some(fid),
        });
    });

    Ok(ScanProgress {
        folder_id,
        total_files: 0, // Actual count comes via events
        processed_files: 0,
        status: "scanning".to_string(),
    })
}

#[tauri::command]
pub async fn get_watched_folders(
    state: State<'_, AppState>,
) -> Result<Vec<WatchedFolder>, AppError> {
    let registry = state.registry.lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let folders: Vec<WatchedFolder> = registry.folders.values().map(|config| {
        WatchedFolder {
            id: config.id.clone(),
            path: config.path.clone(),
            document_count: config.document_count,
            last_scan: config.last_scan.clone().unwrap_or_else(|| "never".to_string()),
            status: if config.is_paused { "paused".to_string() } else { "watching".to_string() },
        }
    }).collect();

    Ok(folders)
}

/// Recursively walk a directory, returning all file paths.
fn walk_dir_recursive(dir: &std::path::Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    if !dir.is_dir() {
        return Ok(files);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(walk_dir_recursive(&path)?);
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}
