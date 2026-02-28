import { useEffect } from "react";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";
import { useOnboardingStore, useSidebarStore } from "@/lib/stores";
import { useWatchedFolders } from "@/hooks/useTauri";
import { cn } from "@/lib/utils";

export function AppShell() {
  const location = useLocation();
  const navigate = useNavigate();
  const { isCompleted: onboardingCompleted } = useOnboardingStore();
  const { isCollapsed } = useSidebarStore();
  const { data: watchedFolders } = useWatchedFolders();

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

  return (
    <div className="h-screen bg-bg-primary">
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
