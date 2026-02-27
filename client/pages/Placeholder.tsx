import { useLocation } from "react-router-dom";

const routeNames: Record<string, string> = {
  "/spaces": "Smart Spaces",
  "/search": "Search",
  "/recent": "Recent Documents",
  "/favorites": "Favorites",
  "/tags": "Tags",
  "/watched": "Watched Folders",
  "/insights": "Insights",
  "/settings": "Settings",
  "/onboarding": "Onboarding",
};

export default function Placeholder() {
  const location = useLocation();
  const routeName = routeNames[location.pathname] || "Page";

  return (
    <div className="flex items-center justify-center min-h-screen">
      <div className="text-center space-y-4">
        <h1 className="page-title text-text-primary">{routeName}</h1>
        <p className="text-text-secondary max-w-sm">
          This page is ready to be built. Continue prompting to fill in this page contents.
        </p>
        <div className="pt-4">
          <div className="inline-block bg-accent-subtle rounded-lg p-4 text-sm text-text-secondary">
            Route: {location.pathname}
          </div>
        </div>
      </div>
    </div>
  );
}
