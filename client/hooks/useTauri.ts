/**
 * React Query hooks for all Tauri IPC commands.
 *
 * Each query hook falls back to mock data in browser mode (isTauri() === false).
 * Each mutation hook uses useMutation with appropriate query invalidation.
 *
 * Usage:
 *   const { data: spaces, isLoading } = useSpaces();
 *   const { data: stats } = useStats();
 *   const { mutate: updateSettings } = useUpdateSettings();
 */

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { tauriInvoke } from "../lib/tauri";
import {
  mockSpaces,
  mockDocuments,
  mockStats,
  mockWatchedFolders,
  mockTags,
  mockSearchResults,
  mockSpaceGraph,
  mockSearchAnalytics,
  mockActivityItems,
  defaultSettings,
} from "../lib/mock-data";
import type {
  Document,
  Space,
  Tag,
  WatchedFolder,
  Stats,
  SearchFilters,
  SearchResult,
  SpaceGraph,
  SearchAnalytics,
  ScanProgress,
  ActivityItem,
  Settings,
  DocumentMeta,
} from "../lib/types";

// --- Query Keys ---------------------------------------------------------------

export const queryKeys = {
  spaces: ["spaces"] as const,
  spaceDocuments: (spaceId: string) => ["spaces", spaceId, "documents"] as const,
  spaceGraph: ["spaces", "graph"] as const,
  document: (id: string) => ["documents", id] as const,
  relatedDocuments: (id: string) => ["documents", id, "related"] as const,
  recentDocuments: ["documents", "recent"] as const,
  favoriteDocuments: ["documents", "favorites"] as const,
  search: (query: string, filters: SearchFilters) => ["search", query, filters] as const,
  searchAnalytics: ["search", "analytics"] as const,
  stats: ["stats"] as const,
  watchedFolders: ["watched-folders"] as const,
  tags: ["tags"] as const,
  activityFeed: ["activity-feed"] as const,
  settings: ["settings"] as const,
};

// --- Space Hooks --------------------------------------------------------------

/**
 * Fetches all Smart Spaces (auto-organized virtual folders).
 */
export function useSpaces() {
  return useQuery({
    queryKey: queryKeys.spaces,
    queryFn: () => tauriInvoke<Space[]>("get_spaces", {}, () => mockSpaces),
  });
}

/**
 * Fetches documents belonging to a specific space.
 */
export function useSpaceDocuments(spaceId: string) {
  return useQuery({
    queryKey: queryKeys.spaceDocuments(spaceId),
    queryFn: () =>
      tauriInvoke<Document[]>(
        "get_space_documents",
        { spaceId },
        () => mockDocuments.filter((d) => d.spaceIds.includes(spaceId)),
      ),
    enabled: Boolean(spaceId),
  });
}

/**
 * Fetches the space relationship graph for visualization.
 */
export function useSpaceGraph() {
  return useQuery({
    queryKey: queryKeys.spaceGraph,
    queryFn: () => tauriInvoke<SpaceGraph>("get_space_graph", {}, () => mockSpaceGraph),
  });
}

/**
 * Triggers a re-clustering of spaces based on current document embeddings.
 */
export function useReclusterSpaces() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => tauriInvoke<Space[]>("recluster_spaces", {}, () => mockSpaces),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.spaces });
      queryClient.invalidateQueries({ queryKey: queryKeys.spaceGraph });
    },
  });
}

/**
 * Moves a document to a different space.
 */
export function useMoveDocumentToSpace() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ docId, spaceId }: { docId: string; spaceId: string }) =>
      tauriInvoke<void>("move_document_to_space", { docId, spaceId }, () => undefined),
    onSuccess: (_data, { spaceId }) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.spaces });
      queryClient.invalidateQueries({ queryKey: queryKeys.spaceDocuments(spaceId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.recentDocuments });
    },
  });
}

// --- Document Hooks -----------------------------------------------------------

/**
 * Fetches a single document by ID.
 */
export function useDocument(id: string) {
  return useQuery({
    queryKey: queryKeys.document(id),
    queryFn: () =>
      tauriInvoke<Document>(
        "get_document",
        { id },
        () => mockDocuments.find((d) => d.id === id) ?? mockDocuments[0],
      ),
    enabled: Boolean(id),
  });
}

/**
 * Fetches documents related to a given document (by embedding similarity).
 */
export function useRelatedDocuments(id: string, limit = 5) {
  return useQuery({
    queryKey: queryKeys.relatedDocuments(id),
    queryFn: () =>
      tauriInvoke<Document[]>(
        "get_related_documents",
        { id, limit },
        () => mockDocuments.filter((d) => d.id !== id).slice(0, limit),
      ),
    enabled: Boolean(id),
  });
}

/**
 * Indexes a new document from a file path.
 */
export function useIndexDocument() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (path: string) =>
      tauriInvoke<DocumentMeta>("index_document", { path }, () => ({
        id: `doc-${Date.now()}`,
        name: path.split("/").pop() ?? "unknown",
        path,
        docType: "other",
        size: 0,
        createdAt: new Date().toISOString(),
        modifiedAt: new Date().toISOString(),
      })),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.stats });
      queryClient.invalidateQueries({ queryKey: queryKeys.recentDocuments });
      queryClient.invalidateQueries({ queryKey: queryKeys.spaces });
    },
  });
}

/**
 * Toggles the favorite status of a document.
 */
export function useToggleFavorite() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (docId: string) =>
      tauriInvoke<void>("toggle_favorite", { docId }, () => undefined),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.favoriteDocuments });
      queryClient.invalidateQueries({ queryKey: queryKeys.recentDocuments });
    },
  });
}

// --- Search Hooks -------------------------------------------------------------

/**
 * Searches documents using semantic similarity.
 * Only fires when query has content.
 */
export function useDocumentSearch(query: string, filters: SearchFilters = {}) {
  return useQuery({
    queryKey: queryKeys.search(query, filters),
    queryFn: () =>
      tauriInvoke<SearchResult[]>(
        "search_documents",
        { query, filters },
        () =>
          mockSearchResults.filter((r) =>
            r.document.name.toLowerCase().includes(query.toLowerCase()),
          ),
      ),
    enabled: query.length > 0,
  });
}

/**
 * Fetches search analytics (top queries, usage stats).
 */
export function useSearchAnalytics() {
  return useQuery({
    queryKey: queryKeys.searchAnalytics,
    queryFn: () =>
      tauriInvoke<SearchAnalytics>("get_search_analytics", {}, () => mockSearchAnalytics),
  });
}

// --- Stats & Activity Hooks ---------------------------------------------------

/**
 * Fetches high-level document and space statistics.
 */
export function useStats() {
  return useQuery({
    queryKey: queryKeys.stats,
    queryFn: () => tauriInvoke<Stats>("get_stats", {}, () => mockStats),
    refetchInterval: 30_000, // poll every 30s to reflect indexing progress
  });
}

/**
 * Fetches recent activity events (indexing, clustering, user actions).
 */
export function useActivityFeed() {
  return useQuery({
    queryKey: queryKeys.activityFeed,
    queryFn: () =>
      tauriInvoke<ActivityItem[]>("get_activity_feed", {}, () => mockActivityItems),
    refetchInterval: 15_000, // poll every 15s
  });
}

// --- Watched Folder Hooks -----------------------------------------------------

/**
 * Fetches the list of watched folders.
 */
export function useWatchedFolders() {
  return useQuery({
    queryKey: queryKeys.watchedFolders,
    queryFn: () =>
      tauriInvoke<WatchedFolder[]>("get_watched_folders", {}, () => mockWatchedFolders),
  });
}

/**
 * Adds a new folder to the watch list.
 */
export function useAddWatchedFolder() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (path: string) =>
      tauriInvoke<WatchedFolder>(
        "add_watched_folder",
        { path },
        () => ({
          id: `folder-${Date.now()}`,
          path,
          documentCount: 0,
          lastScan: new Date().toISOString(),
          status: "watching",
        }),
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.watchedFolders });
    },
  });
}

/**
 * Removes a folder from the watch list.
 */
export function useRemoveWatchedFolder() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) =>
      tauriInvoke<void>("remove_watched_folder", { id }, () => undefined),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.watchedFolders });
    },
  });
}

/**
 * Triggers an immediate scan of a watched folder.
 */
export function useTriggerScan() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (folderId: string) =>
      tauriInvoke<ScanProgress>(
        "trigger_scan",
        { folderId },
        () => ({
          folderId,
          totalFiles: 100,
          processedFiles: 100,
          status: "complete",
        }),
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.watchedFolders });
      queryClient.invalidateQueries({ queryKey: queryKeys.stats });
      queryClient.invalidateQueries({ queryKey: queryKeys.recentDocuments });
    },
  });
}

// --- Tag Hooks ----------------------------------------------------------------

/**
 * Fetches all document tags (auto-generated + user-created).
 */
export function useTags() {
  return useQuery({
    queryKey: queryKeys.tags,
    queryFn: () => tauriInvoke<Tag[]>("get_tags", {}, () => mockTags),
  });
}

// --- Settings Hooks -----------------------------------------------------------

/**
 * Fetches the current application settings.
 */
export function useSettings() {
  return useQuery({
    queryKey: queryKeys.settings,
    queryFn: () => tauriInvoke<Settings>("get_settings", {}, () => defaultSettings),
  });
}

/**
 * Persists updated application settings.
 */
export function useUpdateSettings() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (settings: Settings) =>
      tauriInvoke<void>("update_settings", { settings }, () => undefined),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.settings });
    },
  });
}
