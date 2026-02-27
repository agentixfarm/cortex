use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use crate::engine::CortexEngine;

/// Commands sent to the file watcher background task.
pub enum WatcherCommand {
    Pause,
    Resume,
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
