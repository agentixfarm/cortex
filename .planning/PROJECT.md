# Cortex

## What This Is

Cortex is a Tauri 2 desktop app that auto-organizes personal documents using semantic search, vector embeddings, and GNN-based clustering. Users drop folders, and Cortex automatically creates Smart Spaces (virtual categories) by understanding document content — not just filenames. "Find anything. Organize nothing."

## Core Value

Documents sort themselves into meaningful spaces (Property, Kids, Work, Medical, Invoices) through AI-powered clustering, and users can find anything with natural language search — all running locally on their machine with zero cloud dependency.

## Requirements

### Validated

<!-- Shipped and confirmed valuable. -->

- ✓ React frontend scaffold with 12 routes defined — existing
- ✓ Design system with dark/light mode, custom color tokens, typography — existing
- ✓ 40+ shadcn/ui components adapted to Cortex design language — existing
- ✓ Layout system: AppShell, Sidebar (collapsible), TopBar — existing
- ✓ React Router v7 SPA routing with nested routes — existing
- ✓ Mock data layer for frontend development — existing
- ✓ Build tooling: Vite, TypeScript, TailwindCSS, PostCSS — existing

### Active

<!-- Current scope. Building toward these. -->

- [ ] Tauri 2 desktop shell wrapping the React frontend
- [ ] Rust backend: file watcher (notify-rs) monitoring watched folders
- [ ] Rust backend: document parser (PDF, DOCX, text, markdown, spreadsheets, OCR)
- [ ] Rust backend: embedding engine (ONNX local + optional OpenAI API)
- [ ] RuVector integration: vector storage with HNSW indexing
- [ ] RuVector integration: GNN clustering for automatic Smart Space discovery
- [ ] RuVector integration: graph engine for document relationships
- [ ] RuVector integration: self-learning via SONA engine
- [ ] RuVector integration: metadata filtering (type, date, space, tags)
- [ ] Tauri IPC commands bridging frontend to Rust backend
- [ ] Entity extraction (dates, amounts, people, organizations, locations)
- [ ] Semantic search with natural language queries
- [ ] Smart Spaces: auto-generated virtual folders from GNN clusters
- [ ] Space naming via LLM (local Ollama or API)
- [ ] Watched folders management (add/remove/pause, file type toggles, exclusion patterns)
- [ ] Dashboard with real stats, sparklines, recent docs, top spaces
- [ ] Space detail view with sub-spaces, document list, related spaces
- [ ] Search page with split-pane results + preview, filters
- [ ] Document detail view: preview (65%) + metadata sidebar (35%)
- [ ] Recent documents timeline (Today/Yesterday/This Week)
- [ ] Favorites system (starred documents)
- [ ] Tag system (auto-generated + user-created)
- [ ] Insights/analytics page (donut chart, area chart, space network graph)
- [ ] Settings: General, Indexing, AI & Models, Privacy, Storage, About
- [ ] Command palette (Cmd+K) for search/navigation
- [ ] Keyboard shortcuts (Cmd+1/2/3, Cmd+,, Cmd+D, Cmd+\)
- [ ] Onboarding wizard (Welcome, Select Folders, Scanning, Spaces Ready)
- [ ] System tray with background indexing indicator

### Out of Scope

- Cloud sync / multi-device — local-first only, no cloud required
- Mobile app — desktop-first via Tauri
- Real-time collaboration — single-user desktop app
- Web deployment — Tauri desktop only (existing Netlify config is legacy from prototype)

## Context

**Existing codebase:** React 18 frontend with Express server, currently deployed as web app. All 12 routes use Placeholder page components. Frontend has full design system, 40+ UI components, and mock data layer. Needs conversion from web app to Tauri 2 desktop app.

**RuVector dependency:** RuVector is the self-learning vector database powering all intelligence. Source at `/Users/gshah/work/apps/experiments/ruvector/`. Will be integrated as local Rust crate dependency — 10 crates covering core vectors, GNN, graph, clustering, filtering, collections, domain expansion, attention, and hyperbolic HNSW.

**Frontend spec:** Full 936-line spec at `../FRONTEND_SPEC.md` defining all 12 routes, 40+ components, design system, and interaction patterns.

**Design system:** Master design reference at `../design-system/cortex/MASTER.md`.

**Previous prototype:** Reference implementation at `../cortex-app/` (React 19, mock data only).

## Constraints

- **Privacy**: All document processing must run on-device by default. Content never leaves the machine unless user opts into cloud embeddings.
- **Tech stack**: Tauri 2 (Rust backend) + React 19 + TypeScript + TailwindCSS 4 (frontend). RuVector for all vector/graph/clustering operations.
- **Performance**: Search <100ms (10K docs), index <500ms per doc, cold start <1s, app size <50MB.
- **Embedding models**: ONNX Runtime with all-MiniLM-L6-v2 (384-dim) for local, OpenAI text-embedding-3-small (1536-dim) optional.
- **DOM reparenting**: Use DOM reparenting not React portals for layout persistence (Tauri terminal apps requirement).

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Tauri 2 over Electron | Smaller binary, Rust backend, better performance | — Pending |
| RuVector over standalone vector DB | Custom GNN clustering, self-learning, single binary | — Pending |
| Local-first with optional API | Privacy by default, cloud embeddings optional | — Pending |
| React 18 → React 19 upgrade needed | CLAUDE.md specifies React 19, current code is React 18 | — Pending |
| TailwindCSS 3 → 4 upgrade needed | CLAUDE.md specifies TailwindCSS 4, current code is v3 | — Pending |
| Express server removal | Will be replaced by Tauri IPC, no longer needed | — Pending |

---
*Last updated: 2026-02-27 after initialization*
