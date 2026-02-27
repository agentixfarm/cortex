mod commands;
mod error;
mod state;
mod engine;
mod types;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tauri::Manager;
use state::AppState;
use engine::CortexEngine;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Use Tauri's app_data_dir for proper storage location (platform-appropriate)
            let data_dir = app.path().app_data_dir()
                .expect("could not resolve app data dir")
                .join("vectors");

            let engine = CortexEngine::new_with_path(data_dir)
                .expect("RuVector initialization failed");

            // Placeholder channels — Phase 2+ will connect these to real background tasks.
            let (watcher_tx, _watcher_rx) = mpsc::channel(32);
            let (_index_tx, index_rx) = mpsc::channel(32);

            app.manage(AppState {
                engine: Arc::new(Mutex::new(engine)),
                watcher_tx,
                index_rx: Arc::new(Mutex::new(index_rx)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // documents (5)
            commands::documents::index_document,
            commands::documents::search_documents,
            commands::documents::get_document,
            commands::documents::get_related_documents,
            commands::documents::toggle_favorite,
            // spaces (4)
            commands::spaces::get_spaces,
            commands::spaces::get_space_documents,
            commands::spaces::move_document_to_space,
            commands::spaces::recluster_spaces,
            // folders (4)
            commands::folders::add_watched_folder,
            commands::folders::remove_watched_folder,
            commands::folders::trigger_scan,
            commands::folders::get_watched_folders,
            // analytics (5)
            commands::analytics::get_stats,
            commands::analytics::get_space_graph,
            commands::analytics::get_search_analytics,
            commands::analytics::get_tags,
            commands::analytics::get_activity_feed,
            // settings (2)
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
