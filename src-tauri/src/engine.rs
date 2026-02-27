/// CortexEngine holds all backend state (RuVector collections, filter indices).
/// Phase 1 Plan 04 will add real RuVector fields.
/// Phase 2+ will add document pipeline, file watcher channels.
pub struct CortexEngine {
    // Placeholder — RuVector fields added in Plan 04
}

impl CortexEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}
