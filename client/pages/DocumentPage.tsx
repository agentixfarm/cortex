import { useParams, Link } from "react-router-dom";
import {
  ChevronRight,
  FileText,
  Star,
  ExternalLink,
  Calendar,
  HardDrive,
  Tag,
  Users,
  MapPin,
  DollarSign,
  Clock,
} from "lucide-react";
import {
  ResizablePanelGroup,
  ResizablePanel,
  ResizableHandle,
} from "../components/ui/resizable";
import {
  useDocument,
  useRelatedDocuments,
  useToggleFavorite,
  useSpaces,
} from "../hooks/useTauri";
import { cn } from "../lib/utils";
import { resolveIcon } from "../lib/icons";

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "long",
    day: "numeric",
    year: "numeric",
  });
}

function entityTypeIcon(entityType: string) {
  switch (entityType) {
    case "date":
      return <Calendar size={14} className="text-blue-400" />;
    case "amount":
      return <DollarSign size={14} className="text-green-400" />;
    case "person":
      return <Users size={14} className="text-purple-400" />;
    case "organization":
      return <Users size={14} className="text-amber-400" />;
    case "location":
      return <MapPin size={14} className="text-red-400" />;
    default:
      return <Tag size={14} className="text-text-tertiary" />;
  }
}

function SkeletonDocument() {
  return (
    <div className="h-full animate-pulse p-6 space-y-4">
      <div className="h-6 w-64 rounded bg-bg-tertiary" />
      <div className="h-4 w-48 rounded bg-bg-tertiary" />
      <div className="h-32 rounded bg-bg-tertiary" />
    </div>
  );
}

export default function DocumentPage() {
  const { id } = useParams<{ id: string }>();
  const { data: doc, isLoading, isError } = useDocument(id ?? "");
  const { data: related } = useRelatedDocuments(id ?? "");
  const { data: spaces } = useSpaces();
  const toggleFavorite = useToggleFavorite();

  if (isLoading) return <SkeletonDocument />;

  if (isError || !doc) {
    return (
      <div className="flex items-center justify-center min-h-[60vh]">
        <div className="text-center space-y-2">
          <p className="text-text-primary font-medium">Document not found</p>
          <Link to="/" className="text-sm text-accent-primary hover:text-accent-hover">
            Back to Dashboard
          </Link>
        </div>
      </div>
    );
  }

  // Find first space for breadcrumb
  const primarySpace = spaces?.find((s) => doc.spaceIds.includes(s.id));

  return (
    <div className="h-[calc(100vh-120px)] flex flex-col space-y-4">
      {/* Breadcrumb */}
      <nav className="flex items-center gap-1 text-sm text-text-tertiary">
        <Link to="/" className="hover:text-text-secondary transition-colors">Home</Link>
        {primarySpace && (
          <>
            <ChevronRight size={14} />
            <Link
              to={`/spaces/${primarySpace.id}`}
              className="hover:text-text-secondary transition-colors"
            >
              {primarySpace.name}
            </Link>
          </>
        )}
        <ChevronRight size={14} />
        <span className="text-text-primary font-medium truncate max-w-[200px]">{doc.name}</span>
      </nav>

      {/* Split layout: 65% preview / 35% metadata */}
      <div className="flex-1 min-h-0 rounded-lg border border-border-primary overflow-hidden">
        <ResizablePanelGroup direction="horizontal">
          {/* Preview panel */}
          <ResizablePanel defaultSize={65} minSize={40}>
            <div className="h-full overflow-y-auto p-6 space-y-6">
              {/* Title + type badge */}
              <div className="space-y-3">
                <div className="flex items-start justify-between gap-4">
                  <h1 className="text-2xl font-bold text-text-primary">{doc.name}</h1>
                  <span className="px-2 py-1 text-xs font-medium uppercase rounded bg-accent-subtle text-accent-primary flex-shrink-0">
                    {doc.docType}
                  </span>
                </div>
                <p className="text-sm text-text-tertiary font-mono">{doc.path}</p>
              </div>

              {/* Excerpt */}
              {doc.excerpt && (
                <div className="space-y-2">
                  <h2 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">
                    Content Preview
                  </h2>
                  <div className="rounded-lg bg-bg-secondary border border-border-primary p-4">
                    <p className="text-sm text-text-secondary leading-relaxed whitespace-pre-wrap">
                      {doc.excerpt}
                    </p>
                  </div>
                </div>
              )}

              {/* Open in Finder placeholder */}
              <button
                className="inline-flex items-center gap-1.5 text-sm text-accent-primary hover:text-accent-hover transition-colors"
                onClick={() => {
                  // Future: Tauri shell.open(doc.path)
                }}
              >
                <ExternalLink size={14} />
                Open in Finder
              </button>
            </div>
          </ResizablePanel>

          <ResizableHandle withHandle />

          {/* Metadata sidebar */}
          <ResizablePanel defaultSize={35} minSize={25}>
            <div className="h-full overflow-y-auto p-6 bg-bg-secondary space-y-6">
              {/* Favorite toggle */}
              <button
                onClick={() => toggleFavorite.mutate(doc.id)}
                className={cn(
                  "flex items-center gap-2 px-3 py-2 rounded-lg border transition-colors w-full",
                  doc.isFavorite
                    ? "bg-amber-400/10 border-amber-400/30 text-amber-400"
                    : "bg-bg-tertiary border-border-primary text-text-tertiary hover:text-amber-400",
                )}
              >
                <Star size={16} fill={doc.isFavorite ? "currentColor" : "none"} />
                <span className="text-sm font-medium">
                  {doc.isFavorite ? "Favorited" : "Add to Favorites"}
                </span>
              </button>

              {/* File info */}
              <div className="space-y-3">
                <h3 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">
                  File Info
                </h3>
                <div className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-text-tertiary flex items-center gap-1.5">
                      <FileText size={14} /> Type
                    </span>
                    <span className="text-text-primary uppercase">{doc.docType}</span>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-text-tertiary flex items-center gap-1.5">
                      <HardDrive size={14} /> Size
                    </span>
                    <span className="text-text-primary">{formatBytes(doc.size)}</span>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-text-tertiary flex items-center gap-1.5">
                      <Calendar size={14} /> Created
                    </span>
                    <span className="text-text-primary">{formatDate(doc.createdAt)}</span>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-text-tertiary flex items-center gap-1.5">
                      <Clock size={14} /> Modified
                    </span>
                    <span className="text-text-primary">{formatDate(doc.modifiedAt)}</span>
                  </div>
                </div>
              </div>

              {/* Spaces */}
              {doc.spaceIds.length > 0 && spaces && (
                <div className="space-y-3">
                  <h3 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">
                    Spaces
                  </h3>
                  <div className="space-y-1.5">
                    {spaces
                      .filter((s) => doc.spaceIds.includes(s.id))
                      .map((s) => {
                        const Icon = resolveIcon(s.icon);
                        return (
                          <Link
                            key={s.id}
                            to={`/spaces/${s.id}`}
                            className="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-bg-tertiary transition-colors"
                          >
                            <Icon size={14} style={{ color: s.color }} />
                            <span className="text-sm text-text-primary">{s.name}</span>
                          </Link>
                        );
                      })}
                  </div>
                </div>
              )}

              {/* Tags */}
              {doc.tags.length > 0 && (
                <div className="space-y-3">
                  <h3 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">
                    Tags
                  </h3>
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

              {/* Extracted entities */}
              {doc.extractedEntities.length > 0 && (
                <div className="space-y-3">
                  <h3 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">
                    Extracted Entities
                  </h3>
                  <div className="space-y-2">
                    {doc.extractedEntities.map((e, i) => (
                      <div key={i} className="flex items-center gap-2 text-sm">
                        {entityTypeIcon(e.entityType)}
                        <span className="text-text-tertiary">{e.label}</span>
                        <span className="text-text-primary font-medium ml-auto">{e.value}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Related documents */}
              {related && related.length > 0 && (
                <div className="space-y-3">
                  <h3 className="text-xs font-medium text-text-tertiary uppercase tracking-wider">
                    Related Documents
                  </h3>
                  <div className="space-y-1.5">
                    {related.map((rel) => (
                      <Link
                        key={rel.id}
                        to={`/document/${rel.id}`}
                        className="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-bg-tertiary transition-colors"
                      >
                        <FileText size={14} className="text-text-tertiary flex-shrink-0" />
                        <span className="text-sm text-text-primary truncate">{rel.name}</span>
                      </Link>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
}
