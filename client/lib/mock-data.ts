/**
 * Mock data for frontend development (browser mode, no Tauri runtime).
 * Used by useTauri.ts hooks as fallback when isTauri() returns false.
 */

import type {
  Document,
  Space,
  Tag,
  WatchedFolder,
  Stats,
  SearchResult,
  SpaceGraph,
  SearchAnalytics,
  ActivityItem,
  Settings,
} from "./types";

export const mockStats: Stats = {
  totalDocuments: 3942,
  smartSpaces: 24,
  lastScan: new Date(Date.now() - 2 * 60 * 1000).toISOString(), // 2 min ago
  indexSize: 1_288_490_188, // ~1.2 GB
};

export const mockSpaces: Space[] = [
  {
    id: "space-property",
    name: "Property",
    icon: "Home",
    color: "#8B5CF6",
    documentCount: 124,
    lastUpdated: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000).toISOString(),
    sampleFiles: ["Property_Tax_2025.pdf", "Home_Insurance.pdf"],
    subSpaces: [
      {
        id: "space-property-tax",
        name: "Tax",
        icon: "Receipt",
        color: "#7C3AED",
        documentCount: 34,
        lastUpdated: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
        parentId: "space-property",
      },
      {
        id: "space-property-insurance",
        name: "Insurance",
        icon: "Shield",
        color: "#7C3AED",
        documentCount: 18,
        lastUpdated: new Date(Date.now() - 10 * 24 * 60 * 60 * 1000).toISOString(),
        parentId: "space-property",
      },
    ],
  },
  {
    id: "space-kids",
    name: "Kids",
    icon: "Users",
    color: "#10B981",
    documentCount: 341,
    lastUpdated: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(),
    sampleFiles: ["School_Report.pdf", "Medical_Record.pdf"],
  },
  {
    id: "space-work",
    name: "Work",
    icon: "Briefcase",
    color: "#3B82F6",
    documentCount: 1560,
    lastUpdated: new Date(Date.now() - 30 * 60 * 1000).toISOString(),
    sampleFiles: ["Q4_Report.xlsx", "Project_Plan.docx"],
  },
  {
    id: "space-invoices",
    name: "Invoices",
    icon: "Receipt",
    color: "#F59E0B",
    documentCount: 213,
    lastUpdated: new Date(Date.now() - 6 * 60 * 60 * 1000).toISOString(),
    sampleFiles: ["Invoice_Feb2026.pdf"],
  },
  {
    id: "space-medical",
    name: "Medical",
    icon: "Heart",
    color: "#EF4444",
    documentCount: 87,
    lastUpdated: new Date(Date.now() - 14 * 24 * 60 * 60 * 1000).toISOString(),
    sampleFiles: ["Lab_Results_2025.pdf"],
  },
];

export const mockDocuments: Document[] = [
  {
    id: "doc-1",
    name: "Property_Tax_2025.pdf",
    path: "/Users/demo/Documents/Property/Property_Tax_2025.pdf",
    type: "pdf",
    size: 2_048_576,
    createdAt: "2025-02-15T00:00:00Z",
    modifiedAt: "2025-02-15T00:00:00Z",
    excerpt: "Notice of Property Tax Assessment for fiscal year 2025...",
    spaceIds: ["space-property", "space-property-tax"],
    tags: ["tax", "property", "2025"],
    isFavorite: false,
    extractedEntities: [
      { label: "Year", value: "2025", type: "date" },
      { label: "Amount", value: "$4,200.00", type: "amount" },
    ],
    thumbnailColor: "#8B5CF6",
  },
  {
    id: "doc-2",
    name: "Home_Insurance.pdf",
    path: "/Users/demo/Documents/Property/Home_Insurance.pdf",
    type: "pdf",
    size: 1_572_864,
    createdAt: "2025-01-03T00:00:00Z",
    modifiedAt: "2025-01-03T00:00:00Z",
    excerpt: "Homeowners Insurance Policy — Coverage Summary...",
    spaceIds: ["space-property", "space-property-insurance"],
    tags: ["insurance", "property"],
    isFavorite: true,
    thumbnailColor: "#7C3AED",
  },
  {
    id: "doc-3",
    name: "School_Report.pdf",
    path: "/Users/demo/Documents/Kids/School_Report.pdf",
    type: "pdf",
    size: 524_288,
    createdAt: "2026-02-10T00:00:00Z",
    modifiedAt: "2026-02-10T00:00:00Z",
    excerpt: "Semester progress report for Spring 2026...",
    spaceIds: ["space-kids"],
    tags: ["school", "kids"],
    isFavorite: false,
    thumbnailColor: "#10B981",
  },
  {
    id: "doc-4",
    name: "Invoice_Feb2026.pdf",
    path: "/Users/demo/Documents/Invoices/Invoice_Feb2026.pdf",
    type: "pdf",
    size: 348_160,
    createdAt: "2026-02-20T00:00:00Z",
    modifiedAt: "2026-02-20T00:00:00Z",
    excerpt: "Invoice #INV-2026-0214 for professional services...",
    spaceIds: ["space-invoices"],
    tags: ["invoice", "2026"],
    isFavorite: false,
    extractedEntities: [
      { label: "Amount", value: "$1,500.00", type: "amount" },
      { label: "Date", value: "Feb 20, 2026", type: "date" },
    ],
    thumbnailColor: "#F59E0B",
  },
];

export const mockTags: Tag[] = [
  { id: "tag-tax", name: "tax", color: "#8B5CF6", documentCount: 45, type: "auto" },
  { id: "tag-property", name: "property", color: "#7C3AED", documentCount: 124, type: "auto" },
  { id: "tag-2025", name: "2025", color: "#6D28D9", documentCount: 234, type: "auto" },
  { id: "tag-invoice", name: "invoice", color: "#F59E0B", documentCount: 213, type: "auto" },
  { id: "tag-insurance", name: "insurance", color: "#3B82F6", documentCount: 18, type: "auto" },
  { id: "tag-school", name: "school", color: "#10B981", documentCount: 34, type: "user" },
  { id: "tag-kids", name: "kids", color: "#14B8A6", documentCount: 87, type: "user" },
  { id: "tag-medical", name: "medical", color: "#EF4444", documentCount: 87, type: "auto" },
];

export const mockWatchedFolders: WatchedFolder[] = [
  {
    id: "folder-1",
    path: "/Users/demo/Documents",
    documentCount: 2_340,
    lastScan: new Date(Date.now() - 2 * 60 * 1000).toISOString(),
    status: "watching",
  },
  {
    id: "folder-2",
    path: "/Users/demo/Desktop",
    documentCount: 45,
    lastScan: new Date(Date.now() - 5 * 60 * 1000).toISOString(),
    status: "watching",
  },
  {
    id: "folder-3",
    path: "/Users/demo/Downloads",
    documentCount: 128,
    lastScan: new Date(Date.now() - 15 * 60 * 1000).toISOString(),
    status: "paused",
  },
];

export const mockSearchResults: SearchResult[] = mockDocuments.map((doc, i) => ({
  document: doc,
  score: 0.95 - i * 0.1,
  highlights: [doc.excerpt ?? ""],
}));

export const mockSpaceGraph: SpaceGraph = {
  nodes: mockSpaces.map((s) => ({
    id: s.id,
    name: s.name,
    documentCount: s.documentCount,
    color: s.color,
  })),
  edges: [
    { source: "space-property", target: "space-invoices", weight: 0.6 },
    { source: "space-work", target: "space-invoices", weight: 0.8 },
    { source: "space-kids", target: "space-medical", weight: 0.4 },
  ],
};

export const mockSearchAnalytics: SearchAnalytics = {
  topQueries: [
    { query: "property tax 2025", count: 12 },
    { query: "invoice February", count: 8 },
    { query: "school report spring", count: 5 },
    { query: "medical records", count: 4 },
  ],
  queriesThisWeek: 34,
  avgResultsPerQuery: 8.5,
};

export const mockActivityItems: ActivityItem[] = [
  {
    id: "activity-1",
    action: "3 new documents added today",
    type: "info",
    timestamp: new Date(Date.now() - 30 * 60 * 1000).toISOString(),
  },
  {
    id: "activity-2",
    action: '"Tax 2025" space updated',
    type: "info",
    timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
  },
  {
    id: "activity-3",
    action: "12 documents re-categorized",
    type: "success",
    timestamp: new Date(Date.now() - 4 * 60 * 60 * 1000).toISOString(),
  },
  {
    id: "activity-4",
    action: "Scan completed: ~/Documents",
    type: "success",
    timestamp: new Date(Date.now() - 6 * 60 * 60 * 1000).toISOString(),
  },
];

export const defaultSettings: Settings = {
  indexingEnabled: true,
  watchedPaths: ["/Users/demo/Documents", "/Users/demo/Desktop", "/Users/demo/Downloads"],
  embeddingModel: "local",
  embeddingModelName: "all-MiniLM-L6-v2",
  maxDocumentSizeMb: 100,
  supportedExtensions: ["pdf", "docx", "txt", "md", "xlsx", "csv", "png", "jpg"],
  clusteringEnabled: true,
  clusteringThreshold: 0.75,
  privacyMode: false,
  telemetryEnabled: false,
  storageQuotaGb: 10,
};
