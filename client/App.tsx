import "./global.css";

import { Toaster } from "@/components/ui/toaster";
import { createRoot } from "react-dom/client";
import { Toaster as Sonner } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { AppShell } from "./components/layout/AppShell";
import Index from "./pages/Index";
import Placeholder from "./pages/Placeholder";
import SpacesPage from "./pages/SpacesPage";
import NotFound from "./pages/NotFound";
import { ThemeProvider } from "next-themes";

const queryClient = new QueryClient();

const App = () => (
  <ThemeProvider attribute="class" defaultTheme="dark" enableSystem>
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <Toaster />
        <Sonner />
        <BrowserRouter>
          <Routes>
            <Route element={<AppShell />}>
              <Route path="/" element={<Index />} />
              <Route path="/spaces" element={<SpacesPage />} />
              <Route path="/spaces/:id" element={<Placeholder />} />
              <Route path="/search" element={<Placeholder />} />
              <Route path="/recent" element={<Placeholder />} />
              <Route path="/favorites" element={<Placeholder />} />
              <Route path="/tags" element={<Placeholder />} />
              <Route path="/watched" element={<Placeholder />} />
              <Route path="/insights" element={<Placeholder />} />
              <Route path="/settings" element={<Placeholder />} />
              <Route path="/onboarding" element={<Placeholder />} />
              {/* ADD ALL CUSTOM ROUTES ABOVE THE CATCH-ALL "*" ROUTE */}
            </Route>
            <Route path="*" element={<NotFound />} />
          </Routes>
        </BrowserRouter>
      </TooltipProvider>
    </QueryClientProvider>
  </ThemeProvider>
);

createRoot(document.getElementById("root")!).render(<App />);
