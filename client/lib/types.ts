/**
 * Cortex shared data types used across frontend hooks and components.
 * These mirror the Rust backend types returned by IPC commands.
 */

export interface Document {
  id: string;
  name: string;
  path: string;
  type: "pdf" | "docx" | "txt" | "png" | "jpg" | "xlsx" | "csv" | "md" | "other";
  size: number;
  createdAt: string; // ISO date string
  modifiedAt: string; // ISO date string
  excerpt?: string;
  spaceIds: string[];
  tags: string[];
  isFavorite: boolean;
  extractedEntities?: Array<{
    label: string;
    value: string;
    type: "date" | "amount" | "person" | "organization" | "location";
  }>;
  thumbnailColor?: string;
}

export interface Space {
  id: string;
  name: string;
  icon: string; // Lucide icon name (e.g., 'Home', 'Briefcase')
  color: string; // Hex color for accent
  documentCount: number;
  lastUpdated: string; // ISO date string
  subSpaces?: Space[];
  parentId?: string;
  sampleFiles?: string[];
}

export interface Tag {
  id: string;
  name: string;
  color: string;
  documentCount: number;
  type: "auto" | "user"; // auto = AI-generated, user = manually created
}

export interface WatchedFolder {
  id: string;
  path: string;
  documentCount: number;
  lastScan: string; // ISO date string
  status: "watching" | "paused" | "error";
}

export interface Stats {
  totalDocuments: number;
  smartSpaces: number;
  lastScan: string; // ISO date string
  indexSize: number; // bytes
}

export interface SearchFilters {
  spaceIds?: string[];
  tags?: string[];
  types?: Document["type"][];
  dateRange?: {
    from?: string; // ISO date string
    to?: string; // ISO date string
  };
}

export interface SearchResult {
  document: Document;
  score: number;
  highlights?: string[];
}

export interface SpaceGraph {
  nodes: Array<{
    id: string;
    name: string;
    documentCount: number;
    color: string;
  }>;
  edges: Array<{
    source: string;
    target: string;
    weight: number;
  }>;
}

export interface SearchAnalytics {
  topQueries: Array<{
    query: string;
    count: number;
  }>;
  queriesThisWeek: number;
  avgResultsPerQuery: number;
}

export interface ScanProgress {
  folderId: string;
  total: number;
  processed: number;
  status: "scanning" | "complete" | "error";
}

export interface ActivityItem {
  id: string;
  action: string;
  type: "info" | "success" | "warning" | "error";
  timestamp: string; // ISO date string
  documentId?: string;
}

export interface Settings {
  indexingEnabled: boolean;
  watchedPaths: string[];
  embeddingModel: "local" | "openai" | "ollama";
  embeddingModelName: string;
  apiKey?: string;
  ollamaEndpoint?: string;
  maxDocumentSizeMb: number;
  supportedExtensions: string[];
  clusteringEnabled: boolean;
  clusteringThreshold: number;
  privacyMode: boolean;
  telemetryEnabled: boolean;
  storageQuotaGb: number;
}

export interface DocumentMeta {
  id: string;
  name: string;
  path: string;
  type: Document["type"];
  size: number;
  createdAt: string;
  modifiedAt: string;
}
