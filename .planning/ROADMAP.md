# Roadmap: Cortex

## Overview

Cortex starts as an existing React frontend with mock data and transforms into a fully working Tauri 2 desktop app that indexes documents locally, auto-organizes them into AI-discovered Smart Spaces, and lets users find anything with natural language search. Four phases deliver this: stand up the Tauri shell and type contracts (Phase 1), build the full document ingestion pipeline with file watching (Phase 2), wire GNN clustering and semantic search intelligence (Phase 3), then flip the frontend from mock data to live backend and ship the complete UI (Phase 4).

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Tauri Foundation** - Tauri 2 shell, type contracts, vector storage, and spawn_blocking patterns established before any pipeline code (completed 2026-02-27)
- [x] **Phase 2: Document Pipeline and File Watching** - Full ingestion loop: parse, embed, hash, extract entities, watch folders, index in background (completed 2026-02-28)
- [ ] **Phase 3: Search Intelligence and Smart Spaces** - Semantic search, GNN clustering, graph edges, SONA self-learning, attention re-ranking
- [ ] **Phase 4: Frontend Integration and UX** - All 12 pages wired to live backend, command palette, onboarding, system tray, keyboard shortcuts

## Phase Details

### Phase 1: Tauri Foundation
**Goal**: The Tauri 2 desktop app compiles and runs with all type contracts, IPC plumbing, and vector storage foundations in place — the safe architectural base every subsequent phase builds on.
**Depends on**: Nothing (first phase)
**Requirements**: TAURI-01, TAURI-02, TAURI-03, TAURI-04, TAURI-05, TAURI-06, VSTOR-01, VSTOR-02, VSTOR-03, VSTOR-04
**Success Criteria** (what must be TRUE):
  1. The app launches as a Tauri 2 desktop window showing the React frontend (Express server is gone)
  2. All IPC commands return typed AppError values — no raw strings or panics crossing the bridge
  3. CPU-bound operations use spawn_blocking — dev tools confirm no Tokio runtime blocking during heavy calls
  4. RuVector core is initialized with multi-collection support and metadata filtering ready to receive documents
  5. Frontend hooks operate in mock-data mode by default, switch to Tauri invoke when runtime is present
**Plans**: TBD

### Phase 2: Document Pipeline and File Watching
**Goal**: Documents in watched folders are automatically discovered, parsed, embedded, and indexed — the complete data flow from file on disk to searchable vector in RuVector.
**Depends on**: Phase 1
**Requirements**: DPIP-01, DPIP-02, DPIP-03, DPIP-04, DPIP-05, DPIP-06, DPIP-07, DPIP-08, DPIP-09, FWAT-01, FWAT-02, FWAT-03, FWAT-04, FWAT-05, FWAT-06
**Success Criteria** (what must be TRUE):
  1. User drops a folder of PDFs, DOCXs, and text files — all are indexed within seconds without any manual action
  2. Background indexing progress appears in the UI as files are processed, with no UI freezes
  3. A modified document is re-indexed automatically (old vector replaced); unchanged files are skipped via content hash
  4. Dates, amounts, people, organizations, and locations are extracted and stored as document metadata
  5. Folder exclusions (node_modules, .git, hidden files) and per-folder file-type toggles work as configured
**Plans**: TBD

### Phase 3: Search Intelligence and Smart Spaces
**Goal**: Users can find any indexed document with natural language search, and the system automatically discovers meaningful Smart Spaces through GNN clustering — the defining intelligence of Cortex.
**Depends on**: Phase 2
**Requirements**: SRCH-01, SRCH-02, SRCH-03, SRCH-04, SRCH-05, SRCH-06, SPAC-01, SPAC-02, SPAC-03, SPAC-04, SPAC-05, SPAC-06, SPAC-07, INTL-01, INTL-02, INTL-03, INTL-04
**Success Criteria** (what must be TRUE):
  1. User types a natural language query and gets ranked results with highlighted excerpts showing why each matched
  2. Metadata filters (type, date range, space) narrow search results before vector lookup
  3. Smart Spaces appear automatically after indexing — documents cluster into named groups (Property, Work, Medical) without user configuration
  4. User can move a document to a different space manually; moving one document does not trigger full re-cluster
  5. Related documents panel shows graph-connected documents for any open document
  6. Space network graph data is available for visualization (relationships between spaces)
**Plans**: TBD

### Phase 4: Frontend Integration and UX
**Goal**: Every page in the app shows live data from the Rust backend — mock data is gone, all 12 routes are functional, and the complete UX (onboarding, command palette, keyboard shortcuts, system tray) is operational.
**Depends on**: Phase 3
**Requirements**: PAGE-01, PAGE-02, PAGE-03, PAGE-04, PAGE-05, PAGE-06, PAGE-07, PAGE-08, PAGE-09, PAGE-10, PAGE-11, PAGE-12, UX-01, UX-02, UX-03, UX-04
**Success Criteria** (what must be TRUE):
  1. New user completes the 4-step onboarding wizard, selects a folder, watches the scanning progress, and lands on a populated Spaces view
  2. All 12 pages display live backend data — no mock data visible in any route
  3. Cmd+K command palette opens from any page and navigates to any route or executes any search
  4. Watched folders management page shows real folder status and lets user pause/remove/reconfigure folders
  5. Insights page renders donut chart, area chart, and space network graph from real indexed data
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Tauri Foundation | 5/5 | Complete   | 2026-02-27 |
| 2. Document Pipeline and File Watching | 3/5 | Complete    | 2026-02-28 |
| 3. Search Intelligence and Smart Spaces | 0/TBD | Not started | - |
| 4. Frontend Integration and UX | 0/TBD | Not started | - |
