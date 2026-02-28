import { useEffect } from "react";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";
import { CommandPalette } from "./CommandPalette";
import {
  useOnboardingStore,
  useSidebarStore,
  useCommandPaletteStore,
} from "@/lib/stores";
import { useWatchedFolders } from "@/hooks/useTauri";
import { useTheme } from "next-themes";
import { cn } from "@/lib/utils";

export function AppShell() {
  const location = useLocation();
  const navigate = useNavigate();
  const { isCompleted: onboardingCompleted } = useOnboardingStore();
  const { isCollapsed, toggle: toggleSidebar } = useSidebarStore();
  const { data: watchedFolders } = useWatchedFolders();
  const { theme, setTheme } = useTheme();

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
      // / -> Focus search (when not in input/textarea)
      if (
        e.key === "/" &&
        !meta &&
        !(e.target instanceof HTMLInputElement) &&
        !(e.target instanceof HTMLTextAreaElement)
      ) {
        e.preventDefault();
        useCommandPaletteStore.getState().open();
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
