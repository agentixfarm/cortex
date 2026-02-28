use tauri::State;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn get_stats(
    state: State<'_, AppState>,
) -> Result<Stats, AppError> {
    let _engine = state.engine.clone();
    let result = tokio::task::spawn_blocking(move || {
        // Phase 2 will pull real stats from the RuVector index
        Ok::<Stats, AppError>(Stats {
            total_documents: 0,
            smart_spaces: 0,
            last_scan: "2026-02-27T00:00:00Z".to_string(),
            index_size: 0,
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
    let _engine = state.engine.clone();
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will return tags extracted by the document pipeline
        Ok::<Vec<Tag>, AppError>(vec![])
    })
    .await??;
    Ok(results)
}

#[tauri::command]
pub async fn get_activity_feed(
    state: State<'_, AppState>,
) -> Result<Vec<ActivityItem>, AppError> {
    let _engine = state.engine.clone();
    let results = tokio::task::spawn_blocking(move || {
        // Phase 2 will return a real activity feed from the event log
        Ok::<Vec<ActivityItem>, AppError>(vec![])
    })
    .await??;
    Ok(results)
}
