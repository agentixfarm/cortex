/* eslint-disable @typescript-eslint/ban-ts-comment */
import { useState, useEffect, useCallback } from "react";
import {
  FolderOpen,
  Plus,
  Play,
  Pause,
  Trash2,
  RefreshCw,
  X,
  Loader2,
} from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import {
  useWatchedFolders,
  useAddWatchedFolder,
  useRemoveWatchedFolder,
  useTriggerScan,
} from "../hooks/useTauri";
import { isTauri } from "../lib/tauri";
import type { WatchedFolder } from "../lib/types";

const statusConfig: Record<string, { label: string; className: string }> = {
  watching: { label: "Watching", className: "bg-green-500/10 text-green-400" },
  paused: { label: "Paused", className: "bg-yellow-500/10 text-yellow-400" },
  error: { label: "Error", className: "bg-red-500/10 text-red-400" },
};

interface ScanningState {
  [folderId: string]: boolean;
}

export default function WatchedPage() {
  const { data: folders, isLoading } = useWatchedFolders();
  const { mutate: addFolder, isPending: isAdding } = useAddWatchedFolder();
  const { mutate: removeFolder } = useRemoveWatchedFolder();
  const { mutate: triggerScan } = useTriggerScan();

  const [showAddDialog, setShowAddDialog] = useState(false);
  const [newFolderPath, setNewFolderPath] = useState("");
  const [confirmRemoveId, setConfirmRemoveId] = useState<string | null>(null);
  const [scanning, setScanning] = useState<ScanningState>({});

  // Listen for Tauri index-progress events
  useEffect(() => {
    if (!isTauri()) return;
    let unlisten: (() => void) | undefined;

    (async () => {
      try {
        const { listen } = await import("@tauri-apps/api/event");
        unlisten = await listen<{ folderId: string; status: string }>(
          "index-progress",
          (event) => {
            if (event.payload.status === "complete" || event.payload.status === "error") {
              setScanning((prev) => ({ ...prev, [event.payload.folderId]: false }));
            }
          },
        );
      } catch {
        // Not in Tauri environment
      }
    })();

    return () => {
      unlisten?.();
    };
  }, []);

  const handleAddFolder = useCallback(async () => {
    if (isTauri()) {
      try {
        // @ts-ignore -- @tauri-apps/plugin-dialog may not be installed yet (Tauri-only dep)
        const { open } = await import("@tauri-apps/plugin-dialog");
        const selected = await open({ directory: true, multiple: false });
        if (selected && typeof selected === "string") {
          addFolder(selected, { onSuccess: () => setShowAddDialog(false) });
        }
      } catch {
        // Fallback: show text input
        setShowAddDialog(true);
      }
    } else {
      setShowAddDialog(true);
    }
  }, [addFolder]);

  const handleAddFolderSubmit = useCallback(() => {
    if (!newFolderPath.trim()) return;
    addFolder(newFolderPath.trim(), {
      onSuccess: () => {
        setNewFolderPath("");
        setShowAddDialog(false);
      },
    });
  }, [addFolder, newFolderPath]);

  const handleRemove = useCallback(
    (id: string) => {
      removeFolder(id, {
        onSuccess: () => setConfirmRemoveId(null),
      });
    },
    [removeFolder],
  );

  const handleScan = useCallback(
    (folderId: string) => {
      setScanning((prev) => ({ ...prev, [folderId]: true }));
      triggerScan(folderId, {
        onSuccess: () => {
          setScanning((prev) => ({ ...prev, [folderId]: false }));
        },
        onError: () => {
          setScanning((prev) => ({ ...prev, [folderId]: false }));
        },
      });
    },
    [triggerScan],
  );

  if (isLoading) {
    return (
      <div className="space-y-6">
        <div className="space-y-2">
          <h1 className="page-title text-text-primary">Watched Folders</h1>
          <p className="text-text-secondary">Loading folders...</p>
        </div>
        <div className="space-y-4">
          {Array.from({ length: 3 }).map((_, i) => (
            <div key={i} className="card p-6 animate-pulse">
              <div className="flex items-center gap-4">
                <div className="w-10 h-10 rounded-lg bg-bg-tertiary" />
                <div className="flex-1 space-y-2">
                  <div className="h-4 w-64 rounded bg-bg-tertiary" />
                  <div className="h-3 w-40 rounded bg-bg-tertiary" />
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (!folders || folders.length === 0) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div className="space-y-2">
            <h1 className="page-title text-text-primary">Watched Folders</h1>
            <p className="text-text-secondary">Configure which folders Cortex monitors</p>
          </div>
        </div>
        <div className="flex items-center justify-center min-h-[50vh]">
          <div className="text-center space-y-4">
            <div className="mx-auto w-16 h-16 rounded-full bg-bg-tertiary flex items-center justify-center">
              <FolderOpen size={32} className="text-text-tertiary" />
            </div>
            <h2 className="text-xl font-semibold text-text-primary">No folders being watched</h2>
            <p className="text-text-secondary max-w-sm">
              Add a folder to start discovering and organizing your documents.
            </p>
            <button
              type="button"
              onClick={handleAddFolder}
              className="inline-flex items-center gap-2 mt-2 px-4 py-2 bg-accent-primary text-white rounded-lg hover:bg-accent-hover transition-colors text-sm font-medium"
            >
              <Plus size={16} />
              Add Folder
            </button>
          </div>
        </div>
        {renderAddDialog()}
      </div>
    );
  }

  function renderAddDialog() {
    if (!showAddDialog) return null;
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div className="card p-6 w-full max-w-md space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold text-text-primary">Add Watched Folder</h3>
            <button
              type="button"
              onClick={() => setShowAddDialog(false)}
              className="p-1 rounded-lg hover:bg-bg-tertiary text-text-tertiary"
            >
              <X size={18} />
            </button>
          </div>
          <div>
            <label htmlFor="folder-path" className="text-sm text-text-secondary block mb-1.5">
              Folder path
            </label>
            <input
              id="folder-path"
              type="text"
              value={newFolderPath}
              onChange={(e) => setNewFolderPath(e.target.value)}
              placeholder="/Users/you/Documents"
              className="input-base w-full font-mono text-sm"
              onKeyDown={(e) => e.key === "Enter" && handleAddFolderSubmit()}
            />
          </div>
          <div className="flex justify-end gap-2">
            <button
              type="button"
              onClick={() => setShowAddDialog(false)}
              className="px-3 py-1.5 text-sm text-text-secondary hover:text-text-primary transition-colors"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={handleAddFolderSubmit}
              disabled={!newFolderPath.trim() || isAdding}
              className="px-4 py-1.5 bg-accent-primary text-white rounded-lg hover:bg-accent-hover transition-colors text-sm font-medium disabled:opacity-50"
            >
              {isAdding ? "Adding..." : "Add"}
            </button>
          </div>
        </div>
      </div>
    );
  }

  function renderConfirmDialog(folder: WatchedFolder) {
    if (confirmRemoveId !== folder.id) return null;
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div className="card p-6 w-full max-w-sm space-y-4">
          <h3 className="text-lg font-semibold text-text-primary">Remove Folder</h3>
          <p className="text-sm text-text-secondary">
            Stop watching <span className="font-mono text-text-primary">{folder.path}</span>?
            Documents already indexed will remain in your library.
          </p>
          <div className="flex justify-end gap-2">
            <button
              type="button"
              onClick={() => setConfirmRemoveId(null)}
              className="px-3 py-1.5 text-sm text-text-secondary hover:text-text-primary transition-colors"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={() => handleRemove(folder.id)}
              className="px-4 py-1.5 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors text-sm font-medium"
            >
              Remove
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <h1 className="page-title text-text-primary">Watched Folders</h1>
          <p className="text-text-secondary">
            {folders.length} folder{folders.length !== 1 ? "s" : ""} being monitored
          </p>
        </div>
        <button
          type="button"
          onClick={handleAddFolder}
          className="inline-flex items-center gap-2 px-4 py-2 bg-accent-primary text-white rounded-lg hover:bg-accent-hover transition-colors text-sm font-medium"
        >
          <Plus size={16} />
          Add Folder
        </button>
      </div>

      <div className="space-y-4">
        {folders.map((folder) => {
          const status = statusConfig[folder.status] ?? statusConfig.error;
          const isScanning = scanning[folder.id] ?? false;

          return (
            <div key={folder.id} className="card p-6">
              <div className="flex items-start gap-4">
                <div className="p-2.5 rounded-lg bg-accent-subtle text-accent-primary flex-shrink-0">
                  <FolderOpen size={22} />
                </div>

                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-3">
                    <p className="font-mono text-sm text-text-primary truncate">
                      {folder.path}
                    </p>
                    <span className={`text-xs px-2 py-0.5 rounded-full flex-shrink-0 ${status.className}`}>
                      {status.label}
                    </span>
                  </div>
                  <div className="flex items-center gap-4 mt-2 text-xs text-text-tertiary">
                    <span>{folder.documentCount.toLocaleString()} documents</span>
                    <span>
                      Last scan:{" "}
                      {formatDistanceToNow(new Date(folder.lastScan), { addSuffix: true })}
                    </span>
                  </div>

                  {isScanning && (
                    <div className="mt-3 flex items-center gap-2 text-xs text-accent-primary">
                      <Loader2 size={14} className="animate-spin" />
                      <span>Scanning in progress...</span>
                    </div>
                  )}
                </div>

                <div className="flex items-center gap-1 flex-shrink-0">
                  <button
                    type="button"
                    onClick={() => handleScan(folder.id)}
                    disabled={isScanning}
                    className="p-2 rounded-lg hover:bg-bg-tertiary text-text-tertiary hover:text-text-primary transition-colors disabled:opacity-50"
                    title="Scan now"
                  >
                    {isScanning ? (
                      <Loader2 size={16} className="animate-spin" />
                    ) : (
                      <RefreshCw size={16} />
                    )}
                  </button>
                  {folder.status === "watching" ? (
                    <button
                      type="button"
                      className="p-2 rounded-lg hover:bg-bg-tertiary text-text-tertiary hover:text-text-primary transition-colors"
                      title="Pause (coming soon)"
                      disabled
                    >
                      <Pause size={16} />
                    </button>
                  ) : (
                    <button
                      type="button"
                      className="p-2 rounded-lg hover:bg-bg-tertiary text-text-tertiary hover:text-text-primary transition-colors"
                      title="Resume (coming soon)"
                      disabled
                    >
                      <Play size={16} />
                    </button>
                  )}
                  <button
                    type="button"
                    onClick={() => setConfirmRemoveId(folder.id)}
                    className="p-2 rounded-lg hover:bg-red-500/10 text-text-tertiary hover:text-red-400 transition-colors"
                    title="Remove folder"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              </div>

              {renderConfirmDialog(folder)}
            </div>
          );
        })}
      </div>

      {renderAddDialog()}
    </div>
  );
}
