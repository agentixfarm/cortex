import { Search, Moon, Sun, Loader2 } from "lucide-react";
import { useTheme } from "next-themes";
import { cn } from "@/lib/utils";
import { useCommandPaletteStore, useIndexingStore } from "@/lib/stores";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

export function TopBar() {
  const { theme, setTheme } = useTheme();
  const { open: openPalette } = useCommandPaletteStore();
  const { isIndexing, currentFile, filesProcessed, totalFiles } =
    useIndexingStore();

  return (
    <header className="sticky top-0 z-40 border-b border-border-primary bg-bg-secondary/80 backdrop-blur-sm">
      <div className="flex h-14 items-center justify-between px-6 gap-4">
        {/* Left section - Search bar (opens command palette) */}
        <div className="flex-1 max-w-md">
          <button
            onClick={openPalette}
            className="flex w-full items-center gap-2 rounded-md border border-border-primary bg-bg-primary px-3 py-2 text-sm text-text-tertiary transition-colors hover:bg-bg-tertiary hover:border-border-secondary"
          >
            <Search size={16} />
            <span className="hidden sm:inline">Search documents...</span>
            <kbd className="ml-auto rounded bg-bg-secondary px-1.5 py-0.5 text-[10px] font-mono text-text-tertiary">
              Cmd+K
            </kbd>
          </button>
        </div>

        {/* Right section - Indexing indicator + Theme toggle */}
        <div className="flex items-center gap-2">
          {/* Indexing indicator (UX-04) */}
          {isIndexing && (
            <Tooltip>
              <TooltipTrigger asChild>
                <div className="flex items-center gap-2 rounded-md bg-accent-primary/10 px-2.5 py-1.5 text-xs text-accent-primary">
                  <Loader2 size={14} className="animate-spin" />
                  <span className="hidden sm:inline">
                    Indexing {filesProcessed}/{totalFiles}
                  </span>
                </div>
              </TooltipTrigger>
              <TooltipContent>
                <p className="text-xs">
                  Indexing: {currentFile || "Processing..."}
                </p>
                <p className="text-xs text-muted-foreground">
                  {filesProcessed} of {totalFiles} files
                </p>
              </TooltipContent>
            </Tooltip>
          )}

          {/* Theme toggle */}
          <button
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            className="inline-flex items-center justify-center rounded-md p-2 text-text-secondary hover:bg-bg-tertiary hover:text-text-primary transition-colors"
            aria-label="Toggle theme"
          >
            {theme === "dark" ? <Sun size={18} /> : <Moon size={18} />}
          </button>
        </div>
      </div>
    </header>
  );
}

// TODO (UX-03 System Tray): System tray requires Tauri plugin configuration in Rust:
// 1. Add `tauri-plugin-system-tray` to Cargo.toml
// 2. Configure tray icon and menu in Rust setup code
// 3. Deferred to stretch goal -- TopBar indexing indicator provides the same visibility
