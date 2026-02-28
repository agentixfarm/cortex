use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use crate::engine::CortexEngine;
use crate::pipeline::embedder::EmbeddingService;
use crate::pipeline::indexer::DocumentIndexer;
use crate::spaces::manager::SpaceManager;
use crate::watcher::registry::WatcherRegistry;

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
    /// Send commands to the file watcher.
    pub watcher_tx: mpsc::Sender<WatcherCommand>,
    /// Receive indexing events from background pipeline.
    pub index_rx: Arc<Mutex<mpsc::Receiver<IndexEvent>>>,
    /// Local embedding service (fastembed all-MiniLM-L6-v2, 384-dim).
    pub embedding_service: Arc<EmbeddingService>,
    /// Document indexer orchestrating parse → hash → embed → store.
    pub indexer: Arc<DocumentIndexer>,
    /// Watched folder registry (persists to JSON).
    pub registry: Arc<std::sync::Mutex<WatcherRegistry>>,
    /// Path to watcher-registry.json on disk.
    pub registry_path: PathBuf,
    /// Smart Spaces manager: clustering, naming, manual moves.
    pub space_manager: Arc<std::sync::Mutex<SpaceManager>>,
}
