mod commands;
mod error;
mod state;
mod engine;
mod types;
pub mod pipeline;
pub mod watcher;
pub mod search;
pub mod spaces;
pub mod graph;
pub mod intelligence;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tauri::Manager;
use state::AppState;
use engine::CortexEngine;
use pipeline::embedder::EmbeddingService;
use pipeline::indexer::DocumentIndexer;
use graph::edges::DocumentGraph;
use intelligence::analytics::{ActivityLog, SearchTracker};
use intelligence::sona_bridge::SearchLearner;
use spaces::manager::SpaceManager;
use watcher::registry::WatcherRegistry;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data = app.path().app_data_dir()
                .expect("could not resolve app data dir");
            std::fs::create_dir_all(&app_data)?;

            let data_dir = app_data.join("vectors");
            let registry_path = app_data.join("watcher-registry.json");

            let engine = CortexEngine::new_with_path(data_dir)
                .expect("RuVector initialization failed");

            // Initialize embedding service (downloads ~90MB model on first run)
            let embedding_service = Arc::new(
                EmbeddingService::new_local()
                    .expect("Embedding model init failed — check ~/.cache/fastembed/"),
            );

            // Load persistent watcher registry
            let registry = Arc::new(std::sync::Mutex::new(
                WatcherRegistry::load(&registry_path),
            ));

            // Create document indexer
            let indexer = Arc::new(DocumentIndexer::new());

            // Channels for watcher communication
            let (watcher_tx, watcher_rx) = mpsc::channel(32);
            let (_index_tx, index_rx) = mpsc::channel(32);

            let engine_arc = Arc::new(Mutex::new(engine));

            // Create SpaceManager for Smart Spaces
            let space_manager = Arc::new(std::sync::Mutex::new(SpaceManager::new()));

            // Create DocumentGraph for related documents
            let doc_graph = Arc::new(std::sync::Mutex::new(DocumentGraph::new()));

            // Create SearchLearner (SONA self-learning, 384-dim)
            let search_learner = Arc::new(std::sync::Mutex::new(SearchLearner::new(384)));

            // Create SearchTracker for analytics
            let search_tracker = Arc::new(std::sync::Mutex::new(SearchTracker::new()));

            // Create ActivityLog for activity feed
            let activity_log = Arc::new(std::sync::Mutex::new(ActivityLog::new()));

            // Spawn persistent watcher background task
            let app_handle = app.handle().clone();
            watcher::worker::spawn_watcher_task(
                app_handle,
                engine_arc.clone(),
                embedding_service.clone(),
                indexer.clone(),
                registry.clone(),
                registry_path.clone(),
                watcher_rx,
                activity_log.clone(),
            );

            app.manage(AppState {
                engine: engine_arc,
                watcher_tx,
                index_rx: Arc::new(Mutex::new(index_rx)),
                embedding_service,
                indexer,
                registry,
                registry_path,
                space_manager,
                doc_graph,
                search_learner,
                search_tracker,
                activity_log,
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
            commands::documents::record_search_click,
            commands::documents::get_recent_documents,
            commands::documents::get_favorite_documents,
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
