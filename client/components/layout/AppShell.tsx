import { Outlet } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";

export function AppShell() {
  return (
    <div className="h-screen bg-bg-primary">
      {/* Sidebar - fixed positioned */}
      <Sidebar />

      {/* Main content area with margin to account for fixed sidebar */}
      <div className="ml-60 h-screen flex flex-col">
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
