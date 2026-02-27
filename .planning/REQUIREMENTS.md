# Requirements: Cortex

**Defined:** 2026-02-27
**Core Value:** Documents sort themselves into meaningful spaces through AI-powered clustering, and users find anything with natural language search — all running locally.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Tauri Foundation

- [x] **TAURI-01**: Tauri 2 shell wraps existing React frontend with WebView
- [x] **TAURI-02**: Express server removed, replaced by Tauri IPC command stubs
- [x] **TAURI-03**: AppError enum with serde::Serialize for all IPC error handling
- [ ] **TAURI-04**: spawn_blocking pattern established for all CPU-bound operations
- [ ] **TAURI-05**: Dual-mode frontend hooks (mock data in dev, Tauri invoke in production)
- [x] **TAURI-06**: AppState struct with Arc<CortexEngine> and channel senders

### Document Pipeline

- [ ] **DPIP-01**: PDF text extraction via pdf-extract/lopdf
- [ ] **DPIP-02**: DOCX parsing via docx-rust
- [ ] **DPIP-03**: Plain text and Markdown direct read
- [ ] **DPIP-04**: Spreadsheet indexing (XLSX, CSV) via calamine
- [ ] **DPIP-05**: OCR for images via tesseract bindings (opt-in per folder)
- [ ] **DPIP-06**: Local ONNX embedding generation (all-MiniLM-L6-v2, 384-dim) via fastembed
- [ ] **DPIP-07**: Optional API embedding (OpenAI text-embedding-3-small, 1536-dim)
- [ ] **DPIP-08**: Content hash computation for change detection
- [ ] **DPIP-09**: Entity extraction: dates, amounts, people, organizations, locations

### Vector Storage

- [ ] **VSTOR-01**: RuVector core integration with HNSW indexing
- [ ] **VSTOR-02**: Multi-collection support (separate indices per embedding dimension)
- [ ] **VSTOR-03**: Metadata filtering (type, date range, space, tags) before vector search
- [ ] **VSTOR-04**: Hybrid queries: structured filters + semantic similarity

### File Watching

- [ ] **FWAT-01**: Watched folder monitoring via notify-rs with debounce (300ms)
- [ ] **FWAT-02**: Polling fallback for event-dropped scenarios (notify-rs limitation)
- [ ] **FWAT-03**: File type toggles per watched folder
- [ ] **FWAT-04**: Exclusion patterns (node_modules, .git, hidden files)
- [ ] **FWAT-05**: Background indexing as Tokio task with progress events emitted to frontend
- [ ] **FWAT-06**: Re-index on document modification (content hash comparison)

### Search

- [ ] **SRCH-01**: Semantic search with natural language queries via HNSW nearest neighbor
- [ ] **SRCH-02**: Search result highlighting with matched excerpts
- [ ] **SRCH-03**: Metadata filters (type, date, space) applied pre-search
- [ ] **SRCH-04**: Entity-filtered search ("invoices over $500") using extracted entities
- [ ] **SRCH-05**: Incremental search-as-you-type with 150ms debounce
- [ ] **SRCH-06**: GNN attention re-ranking of search results (ruvector-attention)

### Smart Spaces

- [ ] **SPAC-01**: GNN clustering auto-discovers document groups as Smart Spaces
- [ ] **SPAC-02**: GNN clustering runs as decoupled background job (not per-document)
- [ ] **SPAC-03**: Space naming via rule-based approach (most frequent entity type + noun)
- [ ] **SPAC-04**: Space centroid vectors for similarity comparison
- [ ] **SPAC-05**: Related documents discovery via graph edges (ruvector-graph)
- [ ] **SPAC-06**: User can move document between spaces manually
- [ ] **SPAC-07**: Domain expansion: new spaces bootstrap from existing knowledge (ruvector-domain-expansion)

### Intelligence

- [ ] **INTL-01**: SONA self-learning: search queries generate learning signals
- [ ] **INTL-02**: Click-through data tunes search ranking over time
- [ ] **INTL-03**: Graph edges connect documents by content similarity, shared space, shared tags, shared entities
- [ ] **INTL-04**: Space network graph data from ruvector-graph for visualization

### Frontend Pages

- [ ] **PAGE-01**: Dashboard with real stats (sparklines, recent docs, top spaces, activity feed)
- [ ] **PAGE-02**: Smart Spaces grid (auto-organized, grid/list toggle)
- [ ] **PAGE-03**: Space detail (sub-spaces, document list, related spaces)
- [ ] **PAGE-04**: Search page (split-pane: results + preview panel, filters)
- [ ] **PAGE-05**: Recent documents timeline (Today/Yesterday/This Week)
- [ ] **PAGE-06**: Favorites page (starred documents with sort)
- [ ] **PAGE-07**: Tag cloud page (auto-generated + user-created tags)
- [ ] **PAGE-08**: Watched folders management (add/remove/pause, file type toggles, exclusions)
- [ ] **PAGE-09**: Insights/analytics (donut chart, area chart, bar chart, space network graph)
- [ ] **PAGE-10**: Settings: General, Indexing, AI & Models, Privacy, Storage, About
- [ ] **PAGE-11**: Document detail: preview (65%) + metadata sidebar (35%)
- [ ] **PAGE-12**: Onboarding wizard (Welcome, Select Folders, Scanning, Spaces Ready)

### UX

- [ ] **UX-01**: Command palette (Cmd+K) for search and navigation
- [ ] **UX-02**: Keyboard shortcuts (Cmd+1/2/3, Cmd+,, Cmd+D, Cmd+\)
- [ ] **UX-03**: System tray with background indexing indicator
- [ ] **UX-04**: Background indexing progress in TopBar

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Advanced Spaces

- **ASPAC-01**: Sub-space hierarchy via hyperbolic HNSW clustering
- **ASPAC-02**: LLM-based space naming via Ollama (upgrade from rule-based)
- **ASPAC-03**: Suggest space renames via notification when content shifts

### Distribution

- **DIST-01**: macOS code signing and notarization
- **DIST-02**: Windows MSI installer
- **DIST-03**: Linux AppImage/deb packages
- **DIST-04**: Auto-updater via Tauri plugin

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Auto-move files on disk | Smart Spaces are virtual views only. Moving files breaks symlinks, scripts, references. Trust-destroying if AI is wrong. |
| Cloud sync / multi-device | Breaks local-first guarantee. Complex conflict resolution. Not the product. |
| Real-time collaboration | Single-user desktop app. Adds auth, sync, server complexity. |
| AI chat / Q&A over documents | Separate product surface. RAG chatbot requires LLM integration, hallucination mitigation. |
| Document version history | Git-for-docs is unsolved UX. Storage explosion. Track modified_at instead. |
| Email / IMAP indexing | Massive scope expansion. OAuth flows, protocol complexity. Export to PDF instead. |
| Browser history indexing | Privacy risk. Recall backlash validates users don't want this. ~/Downloads covers most cases. |
| Mandatory cloud embeddings | Breaks local-first trust. Local ONNX always default. Cloud strictly opt-in. |
| Mobile app | Desktop-first via Tauri. Mobile is a separate product. |
| Web deployment | Tauri desktop only. Existing Netlify config is legacy from prototype. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| TAURI-01 | Phase 1 | Complete |
| TAURI-02 | Phase 1 | Complete |
| TAURI-03 | Phase 1 | Complete |
| TAURI-04 | Phase 1 | Pending |
| TAURI-05 | Phase 1 | Pending |
| TAURI-06 | Phase 1 | Complete |
| VSTOR-01 | Phase 1 | Pending |
| VSTOR-02 | Phase 1 | Pending |
| VSTOR-03 | Phase 1 | Pending |
| VSTOR-04 | Phase 1 | Pending |
| DPIP-01 | Phase 2 | Pending |
| DPIP-02 | Phase 2 | Pending |
| DPIP-03 | Phase 2 | Pending |
| DPIP-04 | Phase 2 | Pending |
| DPIP-05 | Phase 2 | Pending |
| DPIP-06 | Phase 2 | Pending |
| DPIP-07 | Phase 2 | Pending |
| DPIP-08 | Phase 2 | Pending |
| DPIP-09 | Phase 2 | Pending |
| FWAT-01 | Phase 2 | Pending |
| FWAT-02 | Phase 2 | Pending |
| FWAT-03 | Phase 2 | Pending |
| FWAT-04 | Phase 2 | Pending |
| FWAT-05 | Phase 2 | Pending |
| FWAT-06 | Phase 2 | Pending |
| SRCH-01 | Phase 3 | Pending |
| SRCH-02 | Phase 3 | Pending |
| SRCH-03 | Phase 3 | Pending |
| SRCH-04 | Phase 3 | Pending |
| SRCH-05 | Phase 3 | Pending |
| SRCH-06 | Phase 3 | Pending |
| SPAC-01 | Phase 3 | Pending |
| SPAC-02 | Phase 3 | Pending |
| SPAC-03 | Phase 3 | Pending |
| SPAC-04 | Phase 3 | Pending |
| SPAC-05 | Phase 3 | Pending |
| SPAC-06 | Phase 3 | Pending |
| SPAC-07 | Phase 3 | Pending |
| INTL-01 | Phase 3 | Pending |
| INTL-02 | Phase 3 | Pending |
| INTL-03 | Phase 3 | Pending |
| INTL-04 | Phase 3 | Pending |
| PAGE-01 | Phase 4 | Pending |
| PAGE-02 | Phase 4 | Pending |
| PAGE-03 | Phase 4 | Pending |
| PAGE-04 | Phase 4 | Pending |
| PAGE-05 | Phase 4 | Pending |
| PAGE-06 | Phase 4 | Pending |
| PAGE-07 | Phase 4 | Pending |
| PAGE-08 | Phase 4 | Pending |
| PAGE-09 | Phase 4 | Pending |
| PAGE-10 | Phase 4 | Pending |
| PAGE-11 | Phase 4 | Pending |
| PAGE-12 | Phase 4 | Pending |
| UX-01 | Phase 4 | Pending |
| UX-02 | Phase 4 | Pending |
| UX-03 | Phase 4 | Pending |
| UX-04 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 58 total (note: original file said 50, actual count is 58)
- Mapped to phases: 58
- Unmapped: 0

---
*Requirements defined: 2026-02-27*
*Last updated: 2026-02-27 after roadmap creation (traceability populated)*
