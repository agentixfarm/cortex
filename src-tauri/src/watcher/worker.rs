use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind};
use notify_debouncer_mini::notify::RecursiveMode;
use serde::Serialize;
use tauri::Emitter;

use crate::engine::CortexEngine;
use crate::pipeline::embedder::EmbeddingService;
use crate::pipeline::indexer::DocumentIndexer;
use crate::state::WatcherCommand;
use crate::watcher::registry::WatcherRegistry;

/// Payload emitted via Tauri event "index-progress" for frontend updates.
#[derive(Clone, Serialize)]
pub struct IndexProgress {
    pub file_path: String,
    pub status: String, // "indexing", "indexed", "skipped", "error", "removed"
    pub doc_id: Option<String>,
    pub error: Option<String>,
    pub folder_id: Option<String>,
}

/// Internal file event passed from notify callback to the async processing loop.
struct FileEvent {
    path: PathBuf,
    kind: FileEventKind,
}

enum FileEventKind {
    CreateOrModify,
    Remove,
}

/// Spawn the file watcher background task. Runs for the lifetime of the app.
///
/// The debouncer uses notify-rs's RecommendedWatcher under the hood, which maps to:
/// - macOS: FSEvents
/// - Linux: inotify
/// - Windows: ReadDirectoryChanges
///
/// FWAT-02 Polling fallback: If RecommendedWatcher fails on network filesystems (NFS, FUSE),
/// notify-rs's PollWatcher can be substituted as a future enhancement. The debouncer accepts
/// any Watcher implementation, so the swap is a one-line change.
pub fn spawn_watcher_task(
    app_handle: tauri::AppHandle,
    engine: Arc<tokio::sync::Mutex<CortexEngine>>,
    embedding_service: Arc<EmbeddingService>,
    indexer: Arc<DocumentIndexer>,
    registry: Arc<std::sync::Mutex<WatcherRegistry>>,
    _registry_path: PathBuf,
    mut cmd_rx: tokio::sync::mpsc::Receiver<WatcherCommand>,
) {
    tauri::async_runtime::spawn(async move {
        // Channel: notify callback (sync) -> async processing loop
        let (file_tx, mut file_rx) = tokio::sync::mpsc::channel::<FileEvent>(256);

        // Create 300ms debouncer
        let file_tx_clone = file_tx.clone();
        let mut debouncer = match new_debouncer(
            Duration::from_millis(300),
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    for event in events {
                        let kind = match event.kind {
                            DebouncedEventKind::Any | DebouncedEventKind::AnyContinuous => {
                                FileEventKind::CreateOrModify
                            }
                            _ => FileEventKind::CreateOrModify,
                        };
                        let _ = file_tx_clone.blocking_send(FileEvent {
                            path: event.path,
                            kind,
                        });
                    }
                }
                Err(e) => {
                    eprintln!("[watcher] notify error: {e}");
                }
            },
        ) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("[watcher] failed to create debouncer: {e}");
                return;
            }
        };

        // Watch all non-paused folders from registry at startup
        {
            let reg = registry.lock().unwrap();
            for config in reg.folders.values() {
                if !config.is_paused {
                    if let Err(e) = debouncer
                        .watcher()
                        .watch(std::path::Path::new(&config.path), RecursiveMode::Recursive)
                    {
                        eprintln!("[watcher] failed to watch {}: {e}", config.path);
                    }
                }
            }
        }

        // Main event loop — debouncer MUST stay alive here
        loop {
            tokio::select! {
                // File change events from notify debouncer
                Some(event) = file_rx.recv() => {
                    let (folder_id, excluded, type_ok) = {
                        let reg = registry.lock().unwrap();
                        match reg.find_folder_for_path(&event.path) {
                            None => (None, true, false),
                            Some(cfg) => {
                                let fid = cfg.id.clone();
                                let excluded = reg.is_excluded(&fid, &event.path);
                                let ext = event.path
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("");
                                let type_ok = reg.is_type_enabled(&fid, ext);
                                (Some(fid), excluded, type_ok)
                            }
                        }
                    };

                    if excluded || !type_ok {
                        continue;
                    }

                    let path_str = event.path.to_string_lossy().to_string();

                    match event.kind {
                        FileEventKind::CreateOrModify => {
                            let _ = app_handle.emit("index-progress", IndexProgress {
                                file_path: path_str.clone(),
                                status: "indexing".to_string(),
                                doc_id: None,
                                error: None,
                                folder_id: folder_id.clone(),
                            });

                            let eng = engine.clone();
                            let emb = embedding_service.clone();
                            let idx = indexer.clone();
                            let ah = app_handle.clone();
                            let ps = path_str.clone();
                            let fid = folder_id.clone();
                            let file_path = event.path.clone();

                            tokio::task::spawn_blocking(move || {
                                let engine_guard = eng.blocking_lock();
                                match idx.index_file(&file_path, &engine_guard, &emb) {
                                    Ok(doc_id) => {
                                        let _ = ah.emit("index-progress", IndexProgress {
                                            file_path: ps,
                                            status: "indexed".to_string(),
                                            doc_id: Some(doc_id),
                                            error: None,
                                            folder_id: fid,
                                        });
                                    }
                                    Err(e) => {
                                        let _ = ah.emit("index-progress", IndexProgress {
                                            file_path: ps,
                                            status: "error".to_string(),
                                            doc_id: None,
                                            error: Some(e.to_string()),
                                            folder_id: fid,
                                        });
                                    }
                                }
                            });
                        }
                        FileEventKind::Remove => {
                            let _ = app_handle.emit("index-progress", IndexProgress {
                                file_path: path_str,
                                status: "removed".to_string(),
                                doc_id: None,
                                error: None,
                                folder_id,
                            });
                        }
                    }
                }

                // Commands from the IPC layer
                Some(cmd) = cmd_rx.recv() => {
                    match cmd {
                        WatcherCommand::AddFolder { path, folder_id: _fid } => {
                            if let Err(e) = debouncer
                                .watcher()
                                .watch(std::path::Path::new(&path), RecursiveMode::Recursive)
                            {
                                eprintln!("[watcher] AddFolder failed for {path}: {e}");
                            }
                        }
                        WatcherCommand::RemoveFolder { folder_id: _fid, path } => {
                            if let Err(e) = debouncer
                                .watcher()
                                .unwatch(std::path::Path::new(&path))
                            {
                                eprintln!("[watcher] RemoveFolder failed for {path}: {e}");
                            }
                        }
                        WatcherCommand::Pause => {
                            let reg = registry.lock().unwrap();
                            for config in reg.folders.values() {
                                let _ = debouncer
                                    .watcher()
                                    .unwatch(std::path::Path::new(&config.path));
                            }
                        }
                        WatcherCommand::Resume => {
                            let reg = registry.lock().unwrap();
                            for config in reg.folders.values() {
                                if !config.is_paused {
                                    let _ = debouncer.watcher().watch(
                                        std::path::Path::new(&config.path),
                                        RecursiveMode::Recursive,
                                    );
                                }
                            }
                        }
                        WatcherCommand::Shutdown => {
                            break;
                        }
                    }
                }

                else => break,
            }
        }

        // debouncer dropped here — all watches stop
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_progress_serializes_to_json() {
        let progress = IndexProgress {
            file_path: "/tmp/doc.pdf".to_string(),
            status: "indexed".to_string(),
            doc_id: Some("abc-123".to_string()),
            error: None,
            folder_id: Some("folder-1".to_string()),
        };
        let json = serde_json::to_string(&progress).unwrap();
        assert!(json.contains("indexed"));
        assert!(json.contains("/tmp/doc.pdf"));
        assert!(json.contains("abc-123"));
    }

    #[test]
    fn test_index_progress_with_error_serializes() {
        let progress = IndexProgress {
            file_path: "/tmp/bad.pdf".to_string(),
            status: "error".to_string(),
            doc_id: None,
            error: Some("parse failed".to_string()),
            folder_id: None,
        };
        let json = serde_json::to_string(&progress).unwrap();
        assert!(json.contains("\"status\":\"error\""));
        assert!(json.contains("parse failed"));
    }
}
