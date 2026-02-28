use tauri::State;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn get_stats(
    state: State<'_, AppState>,
) -> Result<Stats, AppError> {
    let engine = state.engine.clone();
    let space_mgr = state.space_manager.clone();
    let registry = state.registry.clone();

    let result = tokio::task::spawn_blocking(move || {
        let engine_guard = engine.blocking_lock();

        // Count total documents from collection
        let total_documents = match engine_guard.collections.get_collection("documents_384") {
            Some(col) => {
                let col = col.read();
                col.db.keys().map(|k| k.len() as u32).unwrap_or(0)
            }
            None => 0,
        };

        // Count smart spaces
        let smart_spaces = match space_mgr.lock() {
            Ok(mgr) => mgr.space_count() as u32,
            Err(_) => 0,
        };

        // Get last scan from registry
        let last_scan = match registry.lock() {
            Ok(reg) => {
                reg.folders
                    .values()
                    .filter_map(|f| f.last_scan.as_deref())
                    .max()
                    .unwrap_or("never")
                    .to_string()
            }
            Err(_) => "never".to_string(),
        };

        // Estimate index size: total_documents * 384 dimensions * 4 bytes per f32
        let index_size = total_documents as u64 * 384 * 4;

        Ok::<Stats, AppError>(Stats {
            total_documents,
            smart_spaces,
            last_scan,
            index_size,
        })
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn get_space_graph(
    state: State<'_, AppState>,
) -> Result<SpaceGraph, AppError> {
    let graph = state.doc_graph.clone();
    let space_mgr = state.space_manager.clone();

    let result = tokio::task::spawn_blocking(move || {
        let graph_guard = graph
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let space_guard = space_mgr
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok::<SpaceGraph, AppError>(graph_guard.build_space_graph(&space_guard))
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn get_search_analytics(
    state: State<'_, AppState>,
) -> Result<SearchAnalytics, AppError> {
    let tracker = state.search_tracker.clone();

    let result = tokio::task::spawn_blocking(move || {
        let tracker_guard = tracker
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok::<SearchAnalytics, AppError>(tracker_guard.get_analytics())
    })
    .await??;
    Ok(result)
}

#[tauri::command]
pub async fn get_tags(
    state: State<'_, AppState>,
) -> Result<Vec<Tag>, AppError> {
    let engine = state.engine.clone();

    let results = tokio::task::spawn_blocking(move || {
        let engine_guard = engine.blocking_lock();
        let collection_arc = match engine_guard.collections.get_collection("documents_384") {
            Some(col) => col,
            None => return Ok::<Vec<Tag>, AppError>(vec![]),
        };

        let collection = collection_arc.read();
        let all_ids = collection
            .db
            .keys()
            .map_err(|e| AppError::VectorStorage(e.to_string()))?;

        // Collect all tags and count documents per tag
        let mut tag_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

        for id in &all_ids {
            if let Ok(Some(entry)) = collection.db.get(id) {
                if let Some(ref metadata) = entry.metadata {
                    if let Some(tags) = metadata.get("tags").and_then(|v| v.as_array()) {
                        for tag_val in tags {
                            if let Some(tag_name) = tag_val.as_str() {
                                *tag_counts.entry(tag_name.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        // Tag color palette
        let colors = [
            "#6D28D9", "#3B82F6", "#10B981", "#F59E0B", "#EF4444",
            "#8B5CF6", "#EC4899", "#14B8A6", "#F97316", "#6366F1",
        ];

        let mut tags: Vec<Tag> = tag_counts
            .into_iter()
            .enumerate()
            .map(|(i, (name, count))| Tag {
                id: format!("tag-{}", name.to_lowercase().replace(' ', "-")),
                name,
                color: colors[i % colors.len()].to_string(),
                document_count: count,
                tag_type: "auto".to_string(),
            })
            .collect();

        // Sort by document count descending
        tags.sort_by(|a, b| b.document_count.cmp(&a.document_count));

        Ok::<Vec<Tag>, AppError>(tags)
    })
    .await??;
    Ok(results)
}

#[tauri::command]
pub async fn get_activity_feed(
    state: State<'_, AppState>,
) -> Result<Vec<ActivityItem>, AppError> {
    let activity_log = state.activity_log.clone();

    let results = tokio::task::spawn_blocking(move || {
        let log = activity_log
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok::<Vec<ActivityItem>, AppError>(log.recent(50))
    })
    .await??;
    Ok(results)
}
