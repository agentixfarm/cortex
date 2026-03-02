/**
 * Zustand stores for UI state management.
 *
 * Stores:
 *  - useSidebarStore: sidebar collapsed/expanded state
 *  - useCommandPaletteStore: command palette open/close
 *  - useIndexingStore: background indexing progress
 *  - useOnboardingStore: onboarding completion (persisted to localStorage)
 */

import { create } from "zustand";
import { persist } from "zustand/middleware";

// --- Sidebar Store -----------------------------------------------------------

interface SidebarState {
  isCollapsed: boolean;
  toggle: () => void;
  setCollapsed: (collapsed: boolean) => void;
}

export const useSidebarStore = create<SidebarState>((set) => ({
  isCollapsed: false,
  toggle: () => set((s) => ({ isCollapsed: !s.isCollapsed })),
  setCollapsed: (collapsed: boolean) => set({ isCollapsed: collapsed }),
}));

// --- Command Palette Store ---------------------------------------------------

interface CommandPaletteState {
  isOpen: boolean;
  open: () => void;
  close: () => void;
  toggle: () => void;
}

export const useCommandPaletteStore = create<CommandPaletteState>((set) => ({
  isOpen: false,
  open: () => set({ isOpen: true }),
  close: () => set({ isOpen: false }),
  toggle: () => set((s) => ({ isOpen: !s.isOpen })),
}));

// --- Indexing Store ----------------------------------------------------------

interface IndexingState {
  isIndexing: boolean;
  currentFile: string;
  filesProcessed: number;
  totalFiles: number;
  setProgress: (progress: {
    currentFile?: string;
    filesProcessed?: number;
    totalFiles?: number;
    isIndexing?: boolean;
  }) => void;
  reset: () => void;
}

export const useIndexingStore = create<IndexingState>((set) => ({
  isIndexing: false,
  currentFile: "",
  filesProcessed: 0,
  totalFiles: 0,
  setProgress: (progress) =>
    set((s) => ({
      isIndexing: progress.isIndexing ?? s.isIndexing,
      currentFile: progress.currentFile ?? s.currentFile,
      filesProcessed: progress.filesProcessed ?? s.filesProcessed,
      totalFiles: progress.totalFiles ?? s.totalFiles,
    })),
  reset: () =>
    set({
      isIndexing: false,
      currentFile: "",
      filesProcessed: 0,
      totalFiles: 0,
    }),
}));

// --- Onboarding Store (persisted) --------------------------------------------

interface OnboardingState {
  isCompleted: boolean;
  setCompleted: (completed: boolean) => void;
  reset: () => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      isCompleted: false,
      setCompleted: (completed: boolean) => set({ isCompleted: completed }),
      reset: () => set({ isCompleted: false }),
    }),
    {
      name: "cortex-onboarding",
    },
  ),
);
