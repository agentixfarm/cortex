/**
 * Tauri runtime detection and invoke wrapper.
 *
 * Provides dual-mode operation:
 * - In Tauri desktop shell: calls invoke() from @tauri-apps/api/core
 * - In browser (pnpm dev): falls back to the provided mock data function
 */

import { invoke } from "@tauri-apps/api/core";

/**
 * Check if running inside Tauri desktop shell.
 * Returns false in plain browser (pnpm dev without Tauri).
 */
export const isTauri = (): boolean =>
  typeof window !== "undefined" && "__TAURI__" in window;

/**
 * Type-safe invoke wrapper that only calls Tauri when available.
 * Falls back to the provided fallback function in browser mode.
 *
 * @param command - Tauri IPC command name
 * @param args - Arguments to pass to the command
 * @param fallback - Function returning mock data for browser dev mode
 */
export async function tauriInvoke<T>(
  command: string,
  args?: Record<string, unknown>,
  fallback?: () => T | Promise<T>,
): Promise<T> {
  if (isTauri()) {
    return invoke<T>(command, args);
  }
  if (fallback) {
    return fallback();
  }
  throw new Error(`Tauri not available and no fallback for command: ${command}`);
}
