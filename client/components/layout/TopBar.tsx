import { Search, Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import { cn } from "@/lib/utils";

export function TopBar() {
  const { theme, setTheme } = useTheme();

  return (
    <header className="sticky top-0 z-40 border-b border-border-primary bg-bg-secondary/80 backdrop-blur-sm">
      <div className="flex h-14 items-center justify-between px-6 gap-4">
        {/* Left section - Search bar */}
        <div className="flex-1 max-w-md">
          <button className="flex w-full items-center gap-2 rounded-md border border-border-primary bg-bg-primary px-3 py-2 text-sm text-text-tertiary transition-colors hover:bg-bg-tertiary hover:border-border-secondary">
            <Search size={16} />
            <span className="hidden sm:inline">Search documents...</span>
            <span className="sm:hidden">⌘K</span>
          </button>
        </div>

        {/* Right section - Theme toggle */}
        <button
          onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          className="inline-flex items-center justify-center rounded-md p-2 text-text-secondary hover:bg-bg-tertiary hover:text-text-primary transition-colors"
          aria-label="Toggle theme"
        >
          {theme === "dark" ? (
            <Sun size={18} />
          ) : (
            <Moon size={18} />
          )}
        </button>
      </div>
    </header>
  );
}
