import { useState } from "react";
import { Link, useLocation } from "react-router-dom";
import {
  Home,
  Brain,
  Search,
  Clock,
  Star,
  Tag,
  Folder,
  BarChart3,
  Settings,
  ChevronLeft,
  ChevronRight,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { formatBytes } from "@/lib/utils";
import { useSpaces, useStats } from "@/hooks/useTauri";

// Assumed storage quota (configurable in settings later)
const STORAGE_QUOTA_BYTES = 5 * 1024 * 1024 * 1024; // 5 GB

export function Sidebar() {
  const [isCollapsed, setIsCollapsed] = useState(false);
  const location = useLocation();

  // Live data hooks
  const { data: spaces, isLoading: spacesLoading } = useSpaces();
  const { data: stats } = useStats();

  const isActive = (path: string) => {
    return location.pathname === path || location.pathname.startsWith(path + "/");
  };

  // Top 6 spaces by document count
  const sidebarSpaces = spaces
    ? [...spaces].sort((a, b) => b.documentCount - a.documentCount).slice(0, 6)
    : [];

  // Storage display
  const indexSize = stats?.indexSize ?? 0;
  const storageLabel = `${formatBytes(indexSize)} / ${formatBytes(STORAGE_QUOTA_BYTES)}`;
  const storagePercent = STORAGE_QUOTA_BYTES > 0 ? Math.min((indexSize / STORAGE_QUOTA_BYTES) * 100, 100) : 0;

  const mainLinks = [
    { path: "/", label: "Dashboard", icon: Home },
    { path: "/spaces", label: "Smart Spaces", icon: Brain },
    { path: "/search", label: "Search", icon: Search },
    { path: "/recent", label: "Recent", icon: Clock },
    { path: "/favorites", label: "Favorites", icon: Star },
  ];

  const bottomLinks = [
    { path: "/tags", label: "Tags", icon: Tag },
    { path: "/watched", label: "Watched Folders", icon: Folder },
    { path: "/insights", label: "Insights", icon: BarChart3 },
    { path: "/settings", label: "Settings", icon: Settings },
  ];

  const NavLink = ({
    path,
    label,
    icon: Icon,
  }: {
    path: string;
    label: string;
    icon: React.ComponentType<{ size: number; className?: string }>;
  }) => (
    <Link
      to={path}
      className={cn(
        "group relative flex items-center gap-3 rounded-md px-3 py-2.5 text-sm font-medium transition-all duration-150",
        isActive(path)
          ? "bg-accent-primary text-white"
          : "text-text-secondary hover:bg-bg-tertiary hover:text-text-primary"
      )}
    >
      <Icon size={20} />
      {!isCollapsed && <span>{label}</span>}
      {isCollapsed && (
        <div className="absolute left-full ml-2 hidden rounded-md bg-bg-secondary px-2 py-1 text-xs text-text-primary shadow-lg group-hover:block whitespace-nowrap">
          {label}
        </div>
      )}
    </Link>
  );

  return (
    <aside
      className={cn(
        "fixed left-0 top-0 h-screen border-r border-border-primary bg-bg-primary transition-all duration-250 flex flex-col z-50",
        isCollapsed ? "w-20" : "w-60"
      )}
    >
      {/* Logo Section */}
      <div className="flex items-center justify-between gap-2 border-b border-border-primary px-4 py-4">
        <div className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-md bg-accent-primary">
            <Brain size={18} className="text-white" />
          </div>
          {!isCollapsed && (
            <span className="app-title text-text-primary">Cortex</span>
          )}
        </div>
      </div>

      {/* Quick Search — will open command palette in Plan 05 */}
      <div className="border-b border-border-primary px-3 py-4">
        <button
          onClick={() => {
            // Command palette wiring deferred to Plan 05 (UX)
          }}
          className={cn(
            "flex w-full items-center gap-2 rounded-md border border-border-primary bg-bg-secondary px-3 py-2 text-sm text-text-tertiary transition-colors hover:border-border-secondary hover:bg-bg-tertiary",
            isCollapsed && "justify-center"
          )}
        >
          <Search size={16} />
          {!isCollapsed && (
            <span className="flex-1 text-left">Cmd+K</span>
          )}
        </button>
      </div>

      {/* Main Navigation */}
      <nav className="flex-1 space-y-1 overflow-y-auto px-3 py-4">
        <div className="space-y-1">
          {mainLinks.map((link) => (
            <NavLink
              key={link.path}
              path={link.path}
              label={link.label}
              icon={link.icon}
            />
          ))}
        </div>

        {/* Spaces Section — driven by useSpaces() */}
        <div className="pt-4">
          {!isCollapsed && (
            <div className="px-3 py-2">
              <span className="text-xs font-semibold uppercase tracking-wider text-text-tertiary">
                Spaces
              </span>
            </div>
          )}
          <div className="space-y-1">
            {spacesLoading ? (
              // Loading skeleton: 4 placeholder lines
              Array.from({ length: 4 }).map((_, i) => (
                <div
                  key={i}
                  className="flex items-center gap-3 rounded-md px-3 py-2"
                >
                  <div className="h-2 w-2 rounded-full bg-bg-tertiary animate-pulse flex-shrink-0" />
                  {!isCollapsed && (
                    <div className="h-3 flex-1 rounded bg-bg-tertiary animate-pulse" />
                  )}
                </div>
              ))
            ) : sidebarSpaces.length > 0 ? (
              <>
                {sidebarSpaces.map((space) => (
                  <Link
                    key={space.id}
                    to={`/spaces/${space.id}`}
                    className={cn(
                      "group flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-bg-tertiary",
                      isActive(`/spaces/${space.id}`)
                        ? "bg-bg-tertiary text-text-primary"
                        : "text-text-secondary"
                    )}
                  >
                    <div
                      className="h-2 w-2 rounded-full flex-shrink-0"
                      style={{ backgroundColor: space.color }}
                    />
                    {!isCollapsed && (
                      <>
                        <span className="flex-1 truncate">{space.name}</span>
                        <span className="text-xs text-text-tertiary">
                          {space.documentCount}
                        </span>
                      </>
                    )}
                  </Link>
                ))}
                {!isCollapsed && spaces && spaces.length > 6 && (
                  <Link
                    to="/spaces"
                    className="block px-3 py-1.5 text-xs text-accent-primary hover:text-accent-hover transition-colors"
                  >
                    View All ({spaces.length})
                  </Link>
                )}
              </>
            ) : (
              !isCollapsed && (
                <p className="px-3 py-2 text-xs text-text-tertiary">
                  No spaces yet
                </p>
              )
            )}
          </div>
        </div>
      </nav>

      {/* Bottom Section */}
      <div className="border-t border-border-primary px-3 py-4 space-y-1">
        {bottomLinks.map((link) => (
          <NavLink
            key={link.path}
            path={link.path}
            label={link.label}
            icon={link.icon}
          />
        ))}
      </div>

      {/* Storage Bar — real index size from useStats() */}
      <div className="border-t border-border-primary px-3 py-3">
        <div className="space-y-2">
          {!isCollapsed && (
            <div className="text-xs text-text-tertiary">{storageLabel}</div>
          )}
          <div className="h-1.5 w-full rounded-full bg-bg-secondary overflow-hidden">
            <div
              className="h-full rounded-full bg-accent-primary transition-all"
              style={{ width: `${storagePercent}%` }}
            />
          </div>
        </div>
      </div>

      {/* Collapse Toggle */}
      <button
        onClick={() => setIsCollapsed(!isCollapsed)}
        className="hidden sm:flex items-center justify-center border-t border-border-primary py-3 text-text-tertiary hover:text-text-secondary transition-colors w-full"
      >
        {isCollapsed ? (
          <ChevronRight size={18} />
        ) : (
          <ChevronLeft size={18} />
        )}
      </button>
    </aside>
  );
}
