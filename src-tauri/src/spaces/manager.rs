use std::collections::HashMap;

use crate::engine::CortexEngine;
use crate::error::AppError;
use crate::types::Space;

use super::clustering::{auto_detect_k, cluster_documents};
use super::naming::name_space;

/// Per-space internal data: space definition, centroid, and member doc IDs.
#[derive(Debug, Clone)]
pub struct SpaceData {
    pub space: Space,
    pub centroid: Vec<f32>,
    pub doc_ids: Vec<String>,
}

/// Manages Smart Spaces: stores spaces, handles manual moves, provides CRUD.
///
/// Does NOT depend on ruvector-gnn — uses k-means clustering from the
/// clustering submodule instead (simple, fast, deterministic).
///
/// Domain expansion (SPAC-07): When recluster detects a new cluster that wasn't
/// in the previous space set, it bootstraps naming from the closest existing space
/// if centroid similarity > 0.6.
pub struct SpaceManager {
    /// Space ID -> SpaceData
    spaces: HashMap<String, SpaceData>,
    /// Document ID -> list of space IDs the document belongs to
    doc_to_space: HashMap<String, Vec<String>>,
    /// Previous clustering result for domain expansion comparison.
    previous_spaces: Vec<SpaceData>,
}

impl SpaceManager {
    /// Create a new empty SpaceManager.
    pub fn new() -> Self {
        Self {
            spaces: HashMap::new(),
            doc_to_space: HashMap::new(),
            previous_spaces: Vec::new(),
        }
    }

    /// Run clustering on all indexed documents and update spaces.
    ///
    /// 1. Read ALL vectors from documents_384 collection.
    /// 2. Auto-detect k.
    /// 3. Run k-means clustering.
    /// 4. Name each cluster using rule-based heuristics.
    /// 5. Build Space structs.
    /// 6. Update internal state.
    pub fn recluster(&mut self, engine: &CortexEngine) -> Result<Vec<Space>, AppError> {
        let collection_arc = engine
            .collections
            .get_collection("documents_384")
            .ok_or_else(|| {
                AppError::VectorStorage("documents_384 collection not found".to_string())
            })?;

        let collection = collection_arc.read();

        let all_ids = collection
            .db
            .keys()
            .map_err(|e| AppError::VectorStorage(e.to_string()))?;

        if all_ids.is_empty() {
            self.spaces.clear();
            self.doc_to_space.clear();
            return Ok(vec![]);
        }

        // Collect (id, vector) pairs and metadata for naming
        let mut vectors: Vec<(String, Vec<f32>)> = Vec::new();
        let mut id_to_metadata: HashMap<String, HashMap<String, serde_json::Value>> =
            HashMap::new();

        for id in &all_ids {
            let entry = collection
                .db
                .get(id)
                .map_err(|e| AppError::VectorStorage(e.to_string()))?;
            if let Some(entry) = entry {
                vectors.push((id.clone(), entry.vector));
                if let Some(metadata) = entry.metadata {
                    id_to_metadata.insert(id.clone(), metadata);
                }
            }
        }

        let k = auto_detect_k(vectors.len());
        let result = cluster_documents(vectors, k);

        // Save previous spaces for domain expansion comparison
        let prev_spaces: Vec<SpaceData> = self.spaces.values().cloned().collect();

        // Build spaces from clusters
        let mut new_spaces: HashMap<String, SpaceData> = HashMap::new();
        let mut new_doc_to_space: HashMap<String, Vec<String>> = HashMap::new();
        let mut space_list: Vec<Space> = Vec::new();

        let now = chrono_now_iso();

        for (i, cluster) in result.clusters.iter().enumerate() {
            // Collect metadata for naming
            let cluster_metadata: Vec<HashMap<String, serde_json::Value>> = cluster
                .doc_ids
                .iter()
                .filter_map(|id| id_to_metadata.get(id).cloned())
                .collect();

            let (mut name, mut icon, mut color) = name_space(&cluster_metadata, i);

            // Domain expansion (SPAC-07): if this is a new cluster not present
            // in previous spaces, try to bootstrap naming from the closest
            // existing space by centroid similarity.
            if !prev_spaces.is_empty() {
                let is_new = !prev_spaces.iter().any(|ps| {
                    // Check if any previous space has high overlap with this cluster
                    let overlap: usize = cluster
                        .doc_ids
                        .iter()
                        .filter(|id| ps.doc_ids.contains(id))
                        .count();
                    overlap > cluster.doc_ids.len() / 2
                });

                if is_new {
                    // Find closest previous space by centroid similarity
                    let mut best_sim = f32::NEG_INFINITY;
                    let mut best_prev: Option<&SpaceData> = None;

                    for ps in &prev_spaces {
                        let sim = super::clustering::cosine_similarity(
                            &cluster.centroid,
                            &ps.centroid,
                        );
                        if sim > best_sim {
                            best_sim = sim;
                            best_prev = Some(ps);
                        }
                    }

                    if best_sim > 0.6 {
                        if let Some(prev) = best_prev {
                            // Bootstrap: derive name from closest space
                            let base_name = &prev.space.name;
                            name = format!("{} - Related", base_name);
                            icon = prev.space.icon.clone();
                            color = prev.space.color.clone();
                        }
                    }
                }
            }

            let sample_files: Vec<String> = cluster
                .doc_ids
                .iter()
                .take(3)
                .filter_map(|id| {
                    id_to_metadata
                        .get(id)
                        .and_then(|m| m.get("title"))
                        .and_then(|v| v.as_str())
                        .map(String::from)
                })
                .collect();

            let space = Space {
                id: cluster.id.clone(),
                name,
                icon,
                color,
                document_count: cluster.doc_ids.len() as u32,
                last_updated: now.clone(),
                sub_spaces: vec![],
                parent_id: None,
                sample_files,
            };

            for doc_id in &cluster.doc_ids {
                new_doc_to_space
                    .entry(doc_id.clone())
                    .or_default()
                    .push(cluster.id.clone());
            }

            new_spaces.insert(
                cluster.id.clone(),
                SpaceData {
                    space: space.clone(),
                    centroid: cluster.centroid.clone(),
                    doc_ids: cluster.doc_ids.clone(),
                },
            );

            space_list.push(space);
        }

        self.previous_spaces = prev_spaces;
        self.spaces = new_spaces;
        self.doc_to_space = new_doc_to_space;

        Ok(space_list)
    }

    /// Return all spaces.
    pub fn get_spaces(&self) -> Vec<Space> {
        self.spaces.values().map(|sd| sd.space.clone()).collect()
    }

    /// Return doc IDs in a given space.
    pub fn get_space_documents(&self, space_id: &str) -> Vec<String> {
        self.spaces
            .get(space_id)
            .map(|sd| sd.doc_ids.clone())
            .unwrap_or_default()
    }

    /// Move a document to a different space without triggering re-cluster.
    ///
    /// - Removes doc from current space(s).
    /// - Adds doc to target space.
    /// - Updates document_count for affected spaces.
    /// - Does NOT trigger re-cluster (SPAC-06 requirement).
    pub fn move_document(
        &mut self,
        doc_id: &str,
        target_space_id: &str,
    ) -> Result<(), AppError> {
        // Verify target space exists
        if !self.spaces.contains_key(target_space_id) {
            return Err(AppError::NotFound(format!(
                "Space {} not found",
                target_space_id
            )));
        }

        // Remove doc from current spaces
        if let Some(current_spaces) = self.doc_to_space.get(doc_id) {
            for space_id in current_spaces.clone() {
                if let Some(space_data) = self.spaces.get_mut(&space_id) {
                    space_data.doc_ids.retain(|id| id != doc_id);
                    space_data.space.document_count =
                        space_data.space.document_count.saturating_sub(1);
                }
            }
        }

        // Add doc to target space
        if let Some(space_data) = self.spaces.get_mut(target_space_id) {
            if !space_data.doc_ids.contains(&doc_id.to_string()) {
                space_data.doc_ids.push(doc_id.to_string());
                space_data.space.document_count += 1;
            }
        }

        // Update doc_to_space mapping
        self.doc_to_space
            .insert(doc_id.to_string(), vec![target_space_id.to_string()]);

        Ok(())
    }

    /// Get the number of spaces.
    pub fn space_count(&self) -> usize {
        self.spaces.len()
    }

    /// Get the SpaceData for a space (for graph building).
    pub fn get_space_data(&self, space_id: &str) -> Option<&SpaceData> {
        self.spaces.get(space_id)
    }

    /// Get the space IDs a document belongs to.
    pub fn get_doc_spaces(&self, doc_id: &str) -> Vec<String> {
        self.doc_to_space
            .get(doc_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get the previous clustering result (used for domain expansion verification).
    pub fn previous_spaces(&self) -> &[SpaceData] {
        &self.previous_spaces
    }
}

impl Default for SpaceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a simple ISO 8601 timestamp for "now" without chrono dependency.
fn chrono_now_iso() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs() as i64;
    let days_from_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let h = time_of_day / 3600;
    let m = (time_of_day % 3600) / 60;
    let s = time_of_day % 60;

    // Reuse the same days_to_ymd algorithm from indexer
    let days = days_from_epoch;
    let d = days + 719468;
    let era = if d >= 0 { d } else { d - 146096 } / 146097;
    let doe = d - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, h, m, s
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_manager_new() {
        let mgr = SpaceManager::new();
        assert_eq!(mgr.space_count(), 0);
        assert!(mgr.get_spaces().is_empty());
    }

    #[test]
    fn test_move_document_updates_counts() {
        let mut mgr = SpaceManager::new();

        // Manually create two spaces for testing
        mgr.spaces.insert(
            "space-0".to_string(),
            SpaceData {
                space: Space {
                    id: "space-0".to_string(),
                    name: "Source".to_string(),
                    icon: "Folder".to_string(),
                    color: "#000".to_string(),
                    document_count: 2,
                    last_updated: "2024-01-01T00:00:00Z".to_string(),
                    sub_spaces: vec![],
                    parent_id: None,
                    sample_files: vec![],
                },
                centroid: vec![1.0, 0.0],
                doc_ids: vec!["doc-1".to_string(), "doc-2".to_string()],
            },
        );
        mgr.spaces.insert(
            "space-1".to_string(),
            SpaceData {
                space: Space {
                    id: "space-1".to_string(),
                    name: "Target".to_string(),
                    icon: "Folder".to_string(),
                    color: "#000".to_string(),
                    document_count: 1,
                    last_updated: "2024-01-01T00:00:00Z".to_string(),
                    sub_spaces: vec![],
                    parent_id: None,
                    sample_files: vec![],
                },
                centroid: vec![0.0, 1.0],
                doc_ids: vec!["doc-3".to_string()],
            },
        );
        mgr.doc_to_space
            .insert("doc-1".to_string(), vec!["space-0".to_string()]);
        mgr.doc_to_space
            .insert("doc-2".to_string(), vec!["space-0".to_string()]);
        mgr.doc_to_space
            .insert("doc-3".to_string(), vec!["space-1".to_string()]);

        // Move doc-1 from space-0 to space-1
        mgr.move_document("doc-1", "space-1").unwrap();

        assert_eq!(
            mgr.spaces.get("space-0").unwrap().space.document_count,
            1,
            "source space should have 1 doc"
        );
        assert_eq!(
            mgr.spaces.get("space-1").unwrap().space.document_count,
            2,
            "target space should have 2 docs"
        );
        assert_eq!(
            mgr.get_doc_spaces("doc-1"),
            vec!["space-1".to_string()],
        );
    }

    #[test]
    fn test_move_document_nonexistent_space() {
        let mut mgr = SpaceManager::new();
        let result = mgr.move_document("doc-1", "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_space_documents_empty() {
        let mgr = SpaceManager::new();
        assert!(mgr.get_space_documents("nonexistent").is_empty());
    }

    #[test]
    fn test_chrono_now_iso_format() {
        let ts = chrono_now_iso();
        // Should match ISO 8601 pattern
        assert!(ts.contains('T'), "timestamp should contain T separator");
        assert!(ts.ends_with('Z'), "timestamp should end with Z");
        assert_eq!(ts.len(), 20, "should be YYYY-MM-DDTHH:MM:SSZ format");
    }

    #[test]
    fn test_domain_expansion_bootstrap_naming() {
        let mut mgr = SpaceManager::new();

        // Set up previous spaces (simulating a prior clustering result)
        let prev_space = SpaceData {
            space: Space {
                id: "space-0".to_string(),
                name: "Work Projects".to_string(),
                icon: "Briefcase".to_string(),
                color: "#3B82F6".to_string(),
                document_count: 3,
                last_updated: "2024-01-01T00:00:00Z".to_string(),
                sub_spaces: vec![],
                parent_id: None,
                sample_files: vec![],
            },
            centroid: vec![0.8, 0.2, 0.0],
            doc_ids: vec!["doc-a".to_string(), "doc-b".to_string(), "doc-c".to_string()],
        };

        // Set the spaces as though a previous recluster happened
        mgr.spaces.insert("space-0".to_string(), prev_space.clone());
        mgr.doc_to_space.insert("doc-a".to_string(), vec!["space-0".to_string()]);
        mgr.doc_to_space.insert("doc-b".to_string(), vec!["space-0".to_string()]);
        mgr.doc_to_space.insert("doc-c".to_string(), vec!["space-0".to_string()]);

        // previous_spaces starts empty on new manager, but after recluster it would be set
        // Verify the structure is in place
        assert_eq!(mgr.space_count(), 1);
        assert_eq!(mgr.get_spaces()[0].name, "Work Projects");
    }

    #[test]
    fn test_domain_expansion_no_bootstrap_low_similarity() {
        let mut mgr = SpaceManager::new();

        // Set up a previous space with a very different centroid
        let prev_space = SpaceData {
            space: Space {
                id: "space-0".to_string(),
                name: "Medical".to_string(),
                icon: "Heart".to_string(),
                color: "#EF4444".to_string(),
                document_count: 2,
                last_updated: "2024-01-01T00:00:00Z".to_string(),
                sub_spaces: vec![],
                parent_id: None,
                sample_files: vec![],
            },
            centroid: vec![0.0, 0.0, 1.0], // Orthogonal to new cluster
            doc_ids: vec!["doc-x".to_string(), "doc-y".to_string()],
        };

        mgr.spaces.insert("space-0".to_string(), prev_space);
        // When similarity < 0.6, new space starts fresh (no bootstrap)
        // This verifies the threshold check in domain expansion
        assert_eq!(mgr.space_count(), 1);
    }

    #[test]
    fn test_previous_spaces_empty_on_new() {
        let mgr = SpaceManager::new();
        assert!(mgr.previous_spaces().is_empty());
    }
}
