use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use crate::engine::CortexEngine;

/// Commands sent to the file watcher background task.
pub enum WatcherCommand {
    /// Start watching a new folder.
    AddFolder { path: String, folder_id: String },
    /// Stop watching a folder and remove it from active watchers.
    RemoveFolder { folder_id: String, path: String },
    /// Pause watching all folders (unwatch without removing config).
    Pause,
    /// Resume watching all non-paused folders.
    Resume,
    /// Shut down the watcher task cleanly.
    Shutdown,
}

/// Events emitted by the indexing pipeline.
pub enum IndexEvent {
    DocumentIndexed { path: String },
    ScanComplete { folder_id: String },
    Error(String),
}

pub struct AppState {
    pub engine: Arc<Mutex<CortexEngine>>,
    /// Send commands to the file watcher (Phase 2+).
    pub watcher_tx: mpsc::Sender<WatcherCommand>,
    /// Receive indexing events from background pipeline (Phase 2+).
    pub index_rx: Arc<Mutex<mpsc::Receiver<IndexEvent>>>,
}
