import { useState, useEffect, useMemo } from "react";
import { Link } from "react-router-dom";
import {
  Search,
  FileText,
  X,
  ExternalLink,
  Tag,
  Clock,
} from "lucide-react";
import {
  ResizablePanelGroup,
  ResizablePanel,
  ResizableHandle,
} from "../components/ui/resizable";
import { useDocumentSearch, useRecordSearchClick, useSpaces, useTags } from "../hooks/useTauri";
import { cn } from "../lib/utils";
import type { Document, SearchFilters, SearchResult } from "../lib/types";
import { resolveIcon } from "../lib/icons";

// --- Debounce hook ---

function useDebouncedValue<T>(value: T, delayMs: number): T {
  const [debounced, setDebounced] = useState(value);
  useEffect(() => {
    const timer = setTimeout(() => setDebounced(value), delayMs);
    return () => clearTimeout(timer);
  }, [value, delayMs]);
  return debounced;
}

// --- Filter chips ---

const DOC_TYPES = ["pdf", "docx", "txt", "xlsx", "csv", "md", "png", "jpg"];

function FilterChip({
  label,
  active,
  onClick,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "px-3 py-1 rounded-full text-xs font-medium transition-colors border",
        active
          ? "bg-accent-primary text-white border-accent-primary"
          : "bg-bg-secondary text-text-secondary border-border-primary hover:bg-bg-tertiary",
      )}
    >
      {label}
    </button>
  );
}

// --- Score badge ---

function ScoreBadge({ score }: { score: number }) {
  const pct = Math.round(score * 100);
  const color =
    pct >= 80
      ? "text-green-400 bg-green-400/10"
      : pct >= 50
        ? "text-amber-400 bg-amber-400/10"
        : "text-text-tertiary bg-bg-tertiary";
  return (
    <span className={cn("text-xs font-mono px-2 py-0.5 rounded", color)}>
      {pct}%
    </span>
  );
}

// --- Document preview panel ---

function PreviewPanel({ doc }: { doc: Document | null }) {
  if (!doc) {
    return (
      <div className="flex items-center justify-center h-full text-center">
        <div className="space-y-2">
          <Search size={32} className="text-text-tertiary mx-auto" />
          <p className="text-text-secondary text-sm">Select a result to preview</p>
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6 overflow-y-auto h-full">
      <div className="space-y-2">
        <h3 className="text-lg font-semibold text-text-primary">{doc.name}</h3>
        <p className="text-xs text-text-tertiary font-mono">{doc.path}</p>
      </div>

      {doc.excerpt && (
        <div className="space-y-1">
          <h4 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">Excerpt</h4>
          <p className="text-sm text-text-secondary leading-relaxed">{doc.excerpt}</p>
        </div>
      )}

      {doc.extractedEntities.length > 0 && (
        <div className="space-y-2">
          <h4 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">Entities</h4>
          <div className="space-y-1">
            {doc.extractedEntities.map((e, i) => (
              <div key={i} className="flex items-center justify-between text-sm">
                <span className="text-text-tertiary">{e.label}</span>
                <span className="text-text-primary font-medium">{e.value}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {doc.tags.length > 0 && (
        <div className="space-y-2">
          <h4 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">Tags</h4>
          <div className="flex flex-wrap gap-1.5">
            {doc.tags.map((tag) => (
              <span
                key={tag}
                className="px-2 py-0.5 text-xs rounded-full bg-accent-subtle text-accent-primary"
              >
                {tag}
              </span>
            ))}
          </div>
        </div>
      )}

      <div className="pt-2">
        <Link
          to={`/document/${doc.id}`}
          className="inline-flex items-center gap-1.5 text-sm text-accent-primary hover:text-accent-hover transition-colors"
        >
          <ExternalLink size={14} />
          View full document
        </Link>
      </div>
    </div>
  );
}

// --- Skeleton ---

function SkeletonResults() {
  return (
    <div className="space-y-2 p-2">
      {Array.from({ length: 5 }).map((_, i) => (
        <div key={i} className="p-4 rounded-lg bg-bg-secondary animate-pulse">
          <div className="flex items-start gap-3">
            <div className="h-8 w-8 rounded bg-bg-tertiary" />
            <div className="flex-1 space-y-2">
              <div className="h-4 w-48 rounded bg-bg-tertiary" />
              <div className="h-3 w-64 rounded bg-bg-tertiary" />
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}

// --- Main SearchPage ---

export default function SearchPage() {
  const [query, setQuery] = useState("");
  const [selectedDocType, setSelectedDocType] = useState<string | undefined>();
  const [selectedSpaceId, setSelectedSpaceId] = useState<string | undefined>();
  const [selectedResult, setSelectedResult] = useState<Document | null>(null);

  const debouncedQuery = useDebouncedValue(query, 150);

  const filters: SearchFilters = useMemo(
    () => ({
      docType: selectedDocType,
      spaceId: selectedSpaceId,
    }),
    [selectedDocType, selectedSpaceId],
  );

  const { data: results, isLoading, isFetching } = useDocumentSearch(debouncedQuery, filters);
  const { data: spaces } = useSpaces();
  const recordClick = useRecordSearchClick();

  const handleResultClick = (result: SearchResult) => {
    setSelectedResult(result.document);
    if (debouncedQuery) {
      recordClick.mutate({ query: debouncedQuery, documentId: result.document.id });
    }
  };

  return (
    <div className="space-y-4 h-[calc(100vh-120px)] flex flex-col">
      {/* Header */}
      <div>
        <h1 className="page-title text-text-primary">Search</h1>
        <p className="text-text-secondary text-sm mt-1">
          Find documents using natural language queries.
        </p>
      </div>

      {/* Search input */}
      <div className="relative">
        <Search
          size={18}
          className="absolute left-3 top-1/2 -translate-y-1/2 text-text-tertiary"
        />
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search your documents..."
          className="input-base w-full pl-10 pr-10"
          autoFocus
        />
        {query && (
          <button
            onClick={() => {
              setQuery("");
              setSelectedResult(null);
            }}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-text-tertiary hover:text-text-secondary"
          >
            <X size={16} />
          </button>
        )}
      </div>

      {/* Filter bar */}
      <div className="flex flex-wrap items-center gap-2">
        <span className="text-xs text-text-tertiary mr-1">Type:</span>
        {DOC_TYPES.map((t) => (
          <FilterChip
            key={t}
            label={t.toUpperCase()}
            active={selectedDocType === t}
            onClick={() => setSelectedDocType(selectedDocType === t ? undefined : t)}
          />
        ))}
        {spaces && spaces.length > 0 && (
          <>
            <span className="text-xs text-text-tertiary ml-3 mr-1">Space:</span>
            {spaces.slice(0, 5).map((s) => (
              <FilterChip
                key={s.id}
                label={s.name}
                active={selectedSpaceId === s.id}
                onClick={() => setSelectedSpaceId(selectedSpaceId === s.id ? undefined : s.id)}
              />
            ))}
          </>
        )}
      </div>

      {/* Result count / search time */}
      {debouncedQuery && results && (
        <p className="text-xs text-text-tertiary">
          {results.length} result{results.length !== 1 ? "s" : ""}
          {isFetching && " -- searching..."}
        </p>
      )}

      {/* Split pane: results + preview */}
      <div className="flex-1 min-h-0 rounded-lg border border-border-primary overflow-hidden">
        <ResizablePanelGroup direction="horizontal">
          <ResizablePanel defaultSize={60} minSize={30}>
            <div className="h-full overflow-y-auto">
              {!debouncedQuery ? (
                <div className="flex flex-col items-center justify-center h-full text-center space-y-3">
                  <Search size={40} className="text-text-tertiary" />
                  <p className="text-text-secondary">Start typing to search</p>
                </div>
              ) : isLoading ? (
                <SkeletonResults />
              ) : !results || results.length === 0 ? (
                <div className="flex flex-col items-center justify-center h-full text-center space-y-3">
                  <FileText size={40} className="text-text-tertiary" />
                  <p className="text-text-secondary">No results found</p>
                  <p className="text-text-tertiary text-xs">
                    Try different keywords or adjust your filters.
                  </p>
                </div>
              ) : (
                <div className="space-y-1 p-2">
                  {results.map((result) => {
                    const doc = result.document;
                    const isSelected = selectedResult?.id === doc.id;
                    return (
                      <button
                        key={doc.id}
                        onClick={() => handleResultClick(result)}
                        className={cn(
                          "w-full text-left p-4 rounded-lg transition-colors",
                          isSelected
                            ? "bg-accent-subtle border border-accent-primary/30"
                            : "hover:bg-bg-tertiary border border-transparent",
                        )}
                      >
                        <div className="flex items-start gap-3">
                          <FileText size={18} className="text-text-tertiary flex-shrink-0 mt-0.5" />
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2">
                              <span className="font-medium text-text-primary text-sm truncate">
                                {doc.name}
                              </span>
                              <ScoreBadge score={result.score} />
                            </div>
                            {doc.spaceIds.length > 0 && spaces && (
                              <p className="text-xs text-text-tertiary mt-0.5">
                                {spaces
                                  .filter((s) => doc.spaceIds.includes(s.id))
                                  .map((s) => s.name)
                                  .join(", ")}
                              </p>
                            )}
                            {result.matchedExcerpt && (
                              <p className="text-xs text-text-secondary mt-1 line-clamp-2">
                                {result.matchedExcerpt}
                              </p>
                            )}
                          </div>
                        </div>
                      </button>
                    );
                  })}
                </div>
              )}
            </div>
          </ResizablePanel>
          <ResizableHandle withHandle />
          <ResizablePanel defaultSize={40} minSize={25}>
            <div className="h-full bg-bg-secondary">
              <PreviewPanel doc={selectedResult} />
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
}
