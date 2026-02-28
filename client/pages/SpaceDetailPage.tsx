import { useMemo } from "react";
import { Link, useParams } from "react-router-dom";
import { ChevronRight, FileText, FolderOpen } from "lucide-react";
import { useSpaces, useSpaceDocuments } from "../hooks/useTauri";
import { resolveIcon } from "../lib/icons";
import type { Document, Space } from "../lib/types";

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function formatRelativeTime(iso: string): string {
  const ms = Date.now() - new Date(iso).getTime();
  const minutes = Math.floor(ms / 60_000);
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  return `${days}d ago`;
}

function DocTypeIcon({ docType }: { docType: string }) {
  const colorMap: Record<string, string> = {
    pdf: "text-red-400",
    docx: "text-blue-400",
    xlsx: "text-green-400",
    csv: "text-green-400",
    txt: "text-text-tertiary",
    md: "text-text-tertiary",
    png: "text-amber-400",
    jpg: "text-amber-400",
  };
  return <FileText size={16} className={colorMap[docType] ?? "text-text-tertiary"} />;
}

function DocumentRow({ doc }: { doc: Document }) {
  return (
    <Link
      to={`/document/${doc.id}`}
      className="flex items-center gap-4 px-4 py-3 rounded-lg border border-border-primary bg-bg-secondary hover:bg-bg-tertiary transition-colors"
    >
      <DocTypeIcon docType={doc.docType} />
      <span className="font-medium text-text-primary flex-1 truncate">{doc.name}</span>
      <span className="text-xs text-text-tertiary uppercase w-12 text-center">{doc.docType}</span>
      <span className="text-sm text-text-tertiary w-20 text-right">{formatBytes(doc.size)}</span>
      <span className="text-sm text-text-tertiary w-24 text-right">{formatDate(doc.modifiedAt)}</span>
    </Link>
  );
}

function SubSpaceCard({ space }: { space: Space }) {
  const Icon = resolveIcon(space.icon);
  return (
    <Link
      to={`/spaces/${space.id}`}
      className="card p-4 hover:shadow-md hover:border-accent-primary/50 transition-all border-l-4"
      style={{ borderLeftColor: space.color }}
    >
      <div className="flex items-center gap-3">
        <div className="p-2 rounded-lg bg-accent-subtle text-accent-primary">
          <Icon size={18} />
        </div>
        <div>
          <p className="font-medium text-text-primary">{space.name}</p>
          <p className="text-xs text-text-tertiary">{space.documentCount} docs</p>
        </div>
      </div>
    </Link>
  );
}

function SkeletonDetail() {
  return (
    <div className="space-y-6 animate-pulse">
      <div className="flex items-center gap-4">
        <div className="h-12 w-12 rounded-lg bg-bg-tertiary" />
        <div className="space-y-2">
          <div className="h-6 w-40 rounded bg-bg-tertiary" />
          <div className="h-4 w-24 rounded bg-bg-tertiary" />
        </div>
      </div>
      <div className="space-y-2">
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="h-12 rounded-lg bg-bg-tertiary" />
        ))}
      </div>
    </div>
  );
}

export default function SpaceDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { data: spaces, isLoading: spacesLoading } = useSpaces();
  const { data: documents, isLoading: docsLoading } = useSpaceDocuments(id ?? "");

  const space = useMemo(() => {
    if (!spaces || !id) return undefined;
    // Search top-level and sub-spaces
    for (const s of spaces) {
      if (s.id === id) return s;
      for (const sub of s.subSpaces) {
        if (sub.id === id) return sub;
      }
    }
    return undefined;
  }, [spaces, id]);

  const parentSpace = useMemo(() => {
    if (!spaces || !space?.parentId) return undefined;
    return spaces.find((s) => s.id === space.parentId);
  }, [spaces, space]);

  // Related spaces: those that share documents with this space
  const relatedSpaces = useMemo(() => {
    if (!spaces || !documents || !id) return [];
    const relatedIds = new Set<string>();
    for (const doc of documents) {
      for (const sid of doc.spaceIds) {
        if (sid !== id) relatedIds.add(sid);
      }
    }
    return spaces.filter((s) => relatedIds.has(s.id));
  }, [spaces, documents, id]);

  const isLoading = spacesLoading || docsLoading;

  if (isLoading) return <SkeletonDetail />;

  if (!space) {
    return (
      <div className="flex items-center justify-center min-h-[60vh]">
        <div className="text-center space-y-2">
          <p className="text-text-primary font-medium">Space not found</p>
          <Link to="/spaces" className="text-sm text-accent-primary hover:text-accent-hover">
            Back to Spaces
          </Link>
        </div>
      </div>
    );
  }

  const Icon = resolveIcon(space.icon);

  return (
    <div className="space-y-6">
      {/* Breadcrumb */}
      <nav className="flex items-center gap-1 text-sm text-text-tertiary">
        <Link to="/" className="hover:text-text-secondary transition-colors">Home</Link>
        <ChevronRight size={14} />
        <Link to="/spaces" className="hover:text-text-secondary transition-colors">Spaces</Link>
        {parentSpace && (
          <>
            <ChevronRight size={14} />
            <Link to={`/spaces/${parentSpace.id}`} className="hover:text-text-secondary transition-colors">
              {parentSpace.name}
            </Link>
          </>
        )}
        <ChevronRight size={14} />
        <span className="text-text-primary font-medium">{space.name}</span>
      </nav>

      {/* Header */}
      <div className="flex items-center gap-4">
        <div
          className="p-3 rounded-lg"
          style={{ backgroundColor: `${space.color}15`, color: space.color }}
        >
          <Icon size={28} />
        </div>
        <div>
          <h1 className="page-title text-text-primary">{space.name}</h1>
          <p className="text-text-secondary text-sm">
            {space.documentCount} documents -- Updated {formatRelativeTime(space.lastUpdated)}
          </p>
        </div>
      </div>

      {/* Sub-spaces */}
      {space.subSpaces.length > 0 && (
        <div className="space-y-3">
          <h2 className="section-header text-text-primary">Sub-Spaces</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            {space.subSpaces.map((sub) => (
              <SubSpaceCard key={sub.id} space={sub} />
            ))}
          </div>
        </div>
      )}

      {/* Documents */}
      <div className="space-y-3">
        <h2 className="section-header text-text-primary">
          Documents {documents && `(${documents.length})`}
        </h2>
        {!documents || documents.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 text-center space-y-3">
            <FolderOpen size={32} className="text-text-tertiary" />
            <p className="text-text-secondary text-sm">No documents in this space yet.</p>
          </div>
        ) : (
          <div className="space-y-2">
            {documents.map((doc) => (
              <DocumentRow key={doc.id} doc={doc} />
            ))}
          </div>
        )}
      </div>

      {/* Related Spaces */}
      {relatedSpaces.length > 0 && (
        <div className="space-y-3">
          <h2 className="section-header text-text-primary">Related Spaces</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            {relatedSpaces.map((rs) => (
              <SubSpaceCard key={rs.id} space={rs} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
