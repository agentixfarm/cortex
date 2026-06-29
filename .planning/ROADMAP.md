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
- [x] **Phase 3: Search Intelligence and Smart Spaces** - Semantic search, GNN clustering, graph edges, SONA self-learning, attention re-ranking (completed 2026-02-28)
- [x] **Phase 4: Frontend Integration and UX** - All 12 pages wired to live backend, command palette, onboarding, system tray, keyboard shortcuts (completed 2026-02-28)
- [x] **Phase 5: Integration Fixes and Gap Closure** - Fix 6 integration breaks: IPC arg mismatches, event wiring, settings persistence, onboarding layout, path_index rebuild (completed 2026-03-13)
- [ ] **Phase 6: Knowledge Graph and Native Integrations** - Promote entities to first-class graph nodes, add native folder picker, in-app file preview (PDF/image/text), and Open in OS

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
**Plans**: 6 plans
  - Plan 01 (Wave 1): Backend deps, Tauri plugin wiring, capabilities + CSP + asset protocol, ONNX model bundle, frontend deps install (KG-01, KG-05, UX-05, UX-06, PAGE-13 — foundational)
  - Plan 02 (Wave 2): NerService (ort + bert-base-NER) + entities.rs extension (email type fix, dedup-by-pair, NER merge) + types.rs + indexer hook + AppState/lib.rs wiring (KG-01, KG-02)
  - Plan 03 (Wave 3): EntityStore (alias merge, split, related, rename) + 6 entity IPC commands + read_document_text + Tokio backfill task with throttled progress events + Wave 0 fixtures (KG-01..KG-05, PAGE-13)
  - Plan 04 (Wave 2): Frontend types mirror + native folder picker on WatchedPage + DocumentContextMenu + DocumentRow extraction + context-menu wiring on search/recent/favorites/spaces-detail (UX-05, UX-06)
  - Plan 05 (Wave 3): 7 file preview components (FilePreview/PdfPreview/ImagePreview/TextPreview/MarkdownPreview/SizeGuardCard/UnsupportedPreview) + usePreview hook + DocumentPage header buttons + entity-chip-as-Link (PAGE-13, UX-06)
  - Plan 06 (Wave 4): Entity UI (9 components) + EntitiesPage + EntityDetailPage + 7 React Query hooks + BackfillIndicator + useBackfillProgress + Sidebar Entities link + REQUIREMENTS.md update + end-to-end UX checkpoint (KG-01, KG-03, KG-04, KG-05)

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
**Plans**:
  - Plan 01 (Wave 1): Type alignment — serde camelCase on Rust, align TS types, update mock data
  - Plan 02 (Wave 2): Dashboard + layout wiring — Index page, Sidebar, TopBar use live hooks; add missing backend commands
  - Plan 03 (Wave 2): Core pages — Spaces grid, Space detail, Search (split-pane), Document detail
  - Plan 04 (Wave 2): Secondary pages — Recent (timeline), Favorites, Tags (cloud+list), Watched Folders (management)
  - Plan 05 (Wave 2): Analytics + Settings — Insights (4 chart types, network graph), Settings (6 tabs)
  - Plan 06 (Wave 3): UX polish — Onboarding wizard, Cmd+K command palette, keyboard shortcuts, indexing indicator, final cleanup

### Phase 5: Integration Fixes and Gap Closure
**Goal**: Fix all 6 integration breaks so every IPC command works correctly, events flow from backend to frontend, settings persist, and onboarding renders fullscreen.
**Depends on**: Phase 4
**Requirements**: INTL-02, FWAT-05, FWAT-06, PAGE-06, PAGE-08, PAGE-10, PAGE-11, PAGE-12, UX-04
**Gap Closure**: Closes 6 integration breaks from v1.0-MILESTONE-AUDIT.md
**Success Criteria** (what must be TRUE):
  1. toggle_favorite and record_search_click IPC commands succeed at runtime (no arg mismatch errors)
  2. TopBar indexing indicator activates during background indexing (event listener wired)
  3. WatchedPage scan progress updates correctly (field names and status strings match)
  4. Previously-indexed documents are not re-embedded on app restart (path_index rebuilt)
  5. Settings persist across app restarts (JSON file in app_data_dir)
  6. Onboarding wizard renders fullscreen without Sidebar/TopBar
**Plans**:
  - Plan 01: Rust backend fixes — IPC param names, IndexProgress serde camelCase, path_index rebuild
  - Plan 02: Frontend + settings wiring — event listener, settings persistence, onboarding route, WatchedPage fixes

### Phase 6: Knowledge Graph and Native Integrations
**Goal**: Cortex moves from "doc auto-organizer" to "knowledge-graph-backed personal brain" — entities (Property, Person, Organization, Amount, Date) become first-class graph nodes that users can click to see every related document, the native folder picker replaces the manual path text input, and any indexed file can be previewed in-app or opened in the OS default application.
**Depends on**: Phase 5
**Requirements**: KG-01, KG-02, KG-03, KG-04, KG-05, UX-05, PAGE-13, UX-06
**Success Criteria** (what must be TRUE):
  1. Entities extracted from documents appear as graph nodes; clicking an entity surfaces every document mentioning it
  2. Entity normalization merges aliases (e.g., "123 Main St" and "Main Street property") so duplicates collapse
  3. Add Watched Folder opens a native OS folder picker; manual path typing is gone
  4. Document detail page renders an in-app preview for PDF, image, plain-text, and markdown files (not just a 200-char excerpt)
  5. Open in Finder / Open with default app works from Document detail and search results
  6. Knowledge graph is queryable via IPC — frontend can request "entities by type", "documents for entity", "related entities"
**Plans**: 7 plans
  - Plan 01 (Wave 1): Backend deps, Tauri plugin wiring (dialog/opener/fs), capabilities + CSP + asset protocol, ONNX model bundle, frontend deps install (KG-01, KG-05, UX-05, UX-06, PAGE-13 — foundational)
  - Plan 02 (Wave 2): NerService (ort + bert-base-NER) + entities.rs extension (email type fix, dedup-by-pair, NER merge) + types.rs + indexer hook + AppState/lib.rs wiring (KG-01, KG-02)
  - Plan 03 (Wave 3): EntityStore (alias merge, split, related, rename) + 6 entity IPC commands + read_document_text + Tokio backfill task with throttled progress events + Wave 0 fixtures + F1-floor test (KG-01..KG-05, PAGE-13)
  - Plan 04 (Wave 2): Frontend types mirror + native folder picker on WatchedPage (with D-19 client-side directory validation) + DocumentContextMenu + DocumentRow extraction + context-menu wiring on search/recent/favorites/spaces-detail (UX-05, UX-06)
  - Plan 05 (Wave 3): 7 file preview components (FilePreview/PdfPreview/ImagePreview/TextPreview/MarkdownPreview/SizeGuardCard/UnsupportedPreview) + usePreview hook + DocumentPage header buttons + entity-chip-as-Link (PAGE-13, UX-06)
  - Plan 06 (Wave 4): 5 entity components (EntityChip/EntityTypeBadge/EntityCard/EntityTypeFilterBar/RelatedEntityChip) + EntitiesPage + 5 React Query read hooks + Sidebar Entities link + App.tsx /entities route + DocumentPage chip swap + mock-data (KG-01, KG-03, PAGE-13)
  - Plan 07 (Wave 5): EntityDetailPage + 4 entity components (EntityDetailHeader/AliasChipList/AliasChip/SplitAliasDialog) + 2 mutation hooks (rename + split) + BackfillIndicator + useBackfillProgress + useBackfillStore + AppShell mount + App.tsx /entities/:id route + REQUIREMENTS.md update + end-to-end UX checkpoint (KG-01, KG-04, KG-05)

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Tauri Foundation | 5/5 | Complete   | 2026-02-27 |
| 2. Document Pipeline and File Watching | 5/5 | Complete    | 2026-02-28 |
| 3. Search Intelligence and Smart Spaces | 5/5 | Complete    | 2026-02-28 |
| 4. Frontend Integration and UX | 6/6 | Complete | 2026-02-28 |
| 5. Integration Fixes and Gap Closure | 2/2 | Complete   | 2026-03-13 |
| 6. Knowledge Graph and Native Integrations | 0/7 | Planned   | — |
