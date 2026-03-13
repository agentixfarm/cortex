import { useEffect } from "react";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";
import { CommandPalette } from "./CommandPalette";
import {
  useOnboardingStore,
  useSidebarStore,
  useCommandPaletteStore,
  useIndexingStore,
} from "@/lib/stores";
import { useWatchedFolders } from "@/hooks/useTauri";
import { useTheme } from "next-themes";
import { cn } from "@/lib/utils";
import { isTauri } from "@/lib/tauri";

export function AppShell() {
  const location = useLocation();
  const navigate = useNavigate();
  const { isCompleted: onboardingCompleted } = useOnboardingStore();
  const { isCollapsed, toggle: toggleSidebar } = useSidebarStore();
  const { data: watchedFolders } = useWatchedFolders();
  const { theme, setTheme } = useTheme();

  // Bridge Tauri "index-progress" events to useIndexingStore (BREAK 2 fix)
  useEffect(() => {
    if (!isTauri()) return;
    let unlisten: (() => void) | undefined;

    (async () => {
      const { listen } = await import("@tauri-apps/api/event");
      unlisten = await listen<{
        filePath: string;
        status: "indexing" | "indexed" | "skipped" | "error" | "removed" | "complete";
        docId: string | null;
        error: string | null;
        folderId: string | null;
      }>("index-progress", (event) => {
        const { filePath, status } = event.payload;
        if (status === "indexing") {
          useIndexingStore.getState().setProgress({
            isIndexing: true,
            currentFile: filePath,
          });
        } else if (status === "complete") {
          useIndexingStore.getState().setProgress({ isIndexing: false });
          setTimeout(() => {
            useIndexingStore.getState().reset();
          }, 2000);
        } else if (status === "error") {
          useIndexingStore.getState().setProgress({ isIndexing: false });
          setTimeout(() => {
            useIndexingStore.getState().reset();
          }, 2000);
        }
      });
    })();

    return () => {
      unlisten?.();
    };
  }, []);

  // Redirect to onboarding if not completed and no watched folders
  useEffect(() => {
    if (
      !onboardingCompleted &&
      watchedFolders !== undefined &&
      watchedFolders.length === 0 &&
      location.pathname !== "/onboarding"
    ) {
      navigate("/onboarding");
    }
  }, [onboardingCompleted, watchedFolders, location.pathname, navigate]);

  // Global keyboard shortcuts (UX-02)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const meta = e.metaKey || e.ctrlKey;

      // Cmd+K is handled by CommandPalette itself
      if (meta && e.key === "k") return;

      // Cmd+1 -> Dashboard
      if (meta && e.key === "1") {
        e.preventDefault();
        navigate("/");
        return;
      }
      // Cmd+2 -> Spaces
      if (meta && e.key === "2") {
        e.preventDefault();
        navigate("/spaces");
        return;
      }
      // Cmd+3 -> Search
      if (meta && e.key === "3") {
        e.preventDefault();
        navigate("/search");
        return;
      }
      // Cmd+, -> Settings
      if (meta && e.key === ",") {
        e.preventDefault();
        navigate("/settings");
        return;
      }
      // Cmd+D -> Toggle dark mode
      if (meta && e.key === "d") {
        e.preventDefault();
        setTheme(theme === "dark" ? "light" : "dark");
        return;
      }
      // Cmd+\ -> Toggle sidebar
      if (meta && e.key === "\\") {
        e.preventDefault();
        toggleSidebar();
        return;
      }
      // Escape -> Close command palette
      if (e.key === "Escape") {
        useCommandPaletteStore.getState().close();
        return;
      }
      // / -> Navigate to search and focus input (when not in input/textarea)
      if (
        e.key === "/" &&
        !meta &&
        !(e.target instanceof HTMLInputElement) &&
        !(e.target instanceof HTMLTextAreaElement)
      ) {
        e.preventDefault();
        navigate("/search");
        // Focus search input after navigation renders
        setTimeout(() => {
          const searchInput = document.querySelector<HTMLInputElement>('input[placeholder*="Search"]') ??
            document.querySelector<HTMLInputElement>('input[type="search"]') ??
            document.querySelector<HTMLInputElement>('.search-input');
          searchInput?.focus();
        }, 100);
        return;
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [navigate, setTheme, theme, toggleSidebar]);

  return (
    <div className="h-screen bg-bg-primary">
      {/* Command Palette overlay */}
      <CommandPalette />

      {/* Sidebar - fixed positioned */}
      <Sidebar />

      {/* Main content area with margin to account for fixed sidebar */}
      <div
        className={cn(
          "h-screen flex flex-col transition-all duration-250",
          isCollapsed ? "ml-20" : "ml-60",
        )}
      >
        <TopBar />
        <main className="flex-1 overflow-auto">
          <div className="p-6 md:p-8">
            <Outlet />
          </div>
        </main>
      </div>
    </div>
  );
}
