/**
 * Cortex shared data types used across frontend hooks and components.
 * These mirror the Rust backend types returned by IPC commands.
 *
 * Field names match the exact camelCase JSON produced by Rust serde
 * with #[serde(rename_all = "camelCase")].
 */

export interface Document {
  id: string;
  name: string;
  path: string;
  docType: string; // "pdf", "docx", "txt", "png", "jpg", "xlsx", "csv", "md", "other"
  size: number;
  createdAt: string; // ISO date string
  modifiedAt: string; // ISO date string
  excerpt?: string;
  spaceIds: string[];
  tags: string[];
  isFavorite: boolean;
  extractedEntities: Array<{
    label: string;
    value: string;
    entityType: string; // "date", "amount", "person", "organization", "location"
  }>;
  thumbnailColor?: string;
}

export interface DocumentMeta {
  id: string;
  name: string;
  path: string;
  docType: string;
  size: number;
  createdAt: string; // ISO date string
  modifiedAt: string; // ISO date string
}

export interface SearchFilters {
  docType?: string;
  spaceId?: string;
  dateFrom?: string; // ISO date string
  dateTo?: string; // ISO date string
  tags?: string[];
}

export interface SearchResult {
  document: Document;
  score: number;
  matchedExcerpt?: string;
}

export interface Space {
  id: string;
  name: string;
  icon: string; // Lucide icon name (e.g., 'Home', 'Briefcase')
  color: string; // Hex color for accent
  documentCount: number;
  lastUpdated: string; // ISO date string
  subSpaces: Space[];
  parentId?: string;
  sampleFiles: string[];
}

export interface WatchedFolder {
  id: string;
  path: string;
  documentCount: number;
  lastScan: string; // ISO date string
  status: string; // "watching", "paused", "error"
}

export interface ScanProgress {
  folderId: string;
  totalFiles: number;
  processedFiles: number;
  status: string; // "scanning", "complete", "error"
}

export interface Stats {
  totalDocuments: number;
  smartSpaces: number;
  lastScan: string; // ISO date string
  indexSize: number; // bytes
}

export interface SpaceGraph {
  nodes: Array<{
    id: string;
    name: string;
    color: string;
    documentCount: number;
  }>;
  edges: Array<{
    source: string;
    target: string;
    weight: number;
  }>;
}

export interface TopQuery {
  query: string;
  count: number;
}

export interface SearchAnalytics {
  totalSearches: number;
  topQueries: TopQuery[];
  avgResultsPerQuery: number;
  queriesThisWeek: number;
}

export interface Settings {
  theme: string; // "dark", "light", "system"
  sidebarCollapsed: boolean;
  embeddingModel: string; // "local", "openai"
  watchedFolders: string[];
  excludedPatterns: string[];
  indexOnStartup: boolean;
  indexSize: number; // bytes
  storagePath: string;
}

export interface Tag {
  id: string;
  name: string;
  color: string;
  documentCount: number;
  tagType: string; // "auto", "user"
}

export interface ActivityItem {
  id: string;
  action: string; // "indexed", "moved", "tagged", "searched"
  subject: string;
  timestamp: string; // ISO date string
  type: string; // "info", "success", "warning", "error"
  documentId?: string;
}
