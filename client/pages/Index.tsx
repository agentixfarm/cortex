import { Link } from "react-router-dom";
import {
  FileText,
  Brain,
  Clock,
  HardDrive,
  Home,
  Users,
  Briefcase,
  Receipt,
  Activity,
} from "lucide-react";

export default function Dashboard() {
  const stats = [
    {
      label: "Total Documents",
      value: "3.9K",
      icon: FileText,
      color: "bg-blue-500/10 text-blue-500",
    },
    {
      label: "Smart Spaces",
      value: "24",
      icon: Brain,
      color: "bg-purple-500/10 text-purple-500",
    },
    {
      label: "Last Scan",
      value: "2m ago",
      icon: Clock,
      color: "bg-green-500/10 text-green-500",
    },
    {
      label: "Index Size",
      value: "1.2G",
      icon: HardDrive,
      color: "bg-amber-500/10 text-amber-500",
    },
  ];

  const recentDocuments = [
    {
      id: 1,
      name: "Property_Tax_2025.pdf",
      space: "Property > Tax",
      date: "Feb 15",
    },
    {
      id: 2,
      name: "Home_Insurance.pdf",
      space: "Property > Insurance",
      date: "Jan 3",
    },
    {
      id: 3,
      name: "School_Report.pdf",
      space: "Kids > School",
      date: "Feb 10",
    },
    {
      id: 4,
      name: "Invoice_Feb2026.pdf",
      space: "Invoices",
      date: "Feb 20",
    },
  ];

  const topSpaces = [
    {
      id: 1,
      name: "Property",
      count: 12,
      icon: Home,
      color: "border-purple-500",
    },
    {
      id: 2,
      name: "Kids",
      count: 34,
      icon: Users,
      color: "border-green-500",
    },
    {
      id: 3,
      name: "Work",
      count: 156,
      icon: Briefcase,
      color: "border-blue-500",
    },
  ];

  const activityLog = [
    { action: "3 new docs added today", type: "info" },
    { action: '"Tax 2025" space updated', type: "info" },
    { action: "12 docs re-categorized", type: "success" },
  ];

  return (
    <div className="space-y-8">
      {/* Greeting */}
      <div className="space-y-2">
        <h1 className="page-title text-text-primary">Good morning, Gaurav.</h1>
        <p className="text-text-secondary">Here's what's happening with your documents.</p>
      </div>

      {/* Search Bar */}
      <div className="rounded-lg border border-border-primary bg-bg-secondary p-4">
        <input
          type="text"
          placeholder="🔍 Search your documents... (Cmd+K)"
          className="input-base w-full"
        />
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {stats.map((stat) => {
          const Icon = stat.icon;
          return (
            <div key={stat.label} className="card p-6">
              <div className="flex items-start justify-between">
                <div>
                  <p className="text-text-tertiary text-sm font-medium">
                    {stat.label}
                  </p>
                  <p className="text-3xl font-bold text-text-primary mt-2">
                    {stat.value}
                  </p>
                </div>
                <div className={`p-3 rounded-lg ${stat.color}`}>
                  <Icon size={20} />
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Recent Documents Section */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="section-header text-text-primary">Recent Documents</h2>
          <Link
            to="/recent"
            className="text-sm font-medium text-accent-primary hover:text-accent-hover transition-colors"
          >
            View All →
          </Link>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {recentDocuments.map((doc) => (
            <div key={doc.id} className="card p-4 hover:shadow-md transition-shadow">
              <div className="flex items-start gap-3">
                <div className="p-2 rounded-lg bg-accent-subtle text-accent-primary flex-shrink-0">
                  <FileText size={20} />
                </div>
                <div className="flex-1 min-w-0">
                  <p className="font-medium text-text-primary text-sm truncate">
                    {doc.name}
                  </p>
                  <p className="text-xs text-text-tertiary mt-1">{doc.space}</p>
                  <p className="text-xs text-text-tertiary mt-2">{doc.date}</p>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Top Spaces Section */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="section-header text-text-primary">Top Spaces</h2>
          <Link
            to="/spaces"
            className="text-sm font-medium text-accent-primary hover:text-accent-hover transition-colors"
          >
            View All →
          </Link>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {topSpaces.map((space) => {
            const IconComponent = space.icon;
            return (
              <Link
                key={space.id}
                to={`/spaces/${space.name.toLowerCase()}`}
                className="card p-6 hover:shadow-lg hover:border-accent-primary/50 transition-all border-l-4"
                style={{
                  borderLeftColor: `var(--${space.color.split("-")[1]}-500)`,
                }}
              >
                <div className="space-y-3">
                  <div className="flex items-start justify-between">
                    <div className="p-2 rounded-lg bg-accent-subtle text-accent-primary">
                      <IconComponent size={24} />
                    </div>
                  </div>
                  <div>
                    <p className="font-semibold text-text-primary text-lg">
                      {space.name}
                    </p>
                    <p className="text-text-tertiary text-sm">
                      {space.count} documents
                    </p>
                  </div>
                </div>
              </Link>
            );
          })}
        </div>
      </div>

      {/* Activity Timeline */}
      <div className="space-y-4">
        <h2 className="section-header text-text-primary">Activity</h2>
        <div className="card p-6">
          <div className="space-y-4">
            {activityLog.map((item, idx) => (
              <div key={idx} className="flex items-center gap-3">
                <div
                  className={`h-2 w-2 rounded-full ${
                    item.type === "success" ? "bg-success" : "bg-info"
                  }`}
                />
                <p className="text-text-secondary text-sm">{item.action}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
