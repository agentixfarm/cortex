mod error;
mod state;
mod engine;
mod types;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use state::AppState;
use engine::CortexEngine;

pub fn run() {
    let engine = CortexEngine::new().expect("CortexEngine initialization failed");

    // Placeholder channels — Phase 2+ will connect these to real background tasks.
    let (watcher_tx, _watcher_rx) = mpsc::channel(32);
    let (_index_tx, index_rx) = mpsc::channel(32);

    tauri::Builder::default()
        .manage(AppState {
            engine: Arc::new(Mutex::new(engine)),
            watcher_tx,
            index_rx: Arc::new(Mutex::new(index_rx)),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
