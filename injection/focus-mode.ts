import { invoke } from '@tauri-apps/api/core';
import { logger } from './lib/logger';

const log = logger('injection::focus-mode');
const POLL_INTERVAL = 5000;

let cached = false;

export function isFocusModeActive(): boolean {
  return cached;
}

async function refresh(): Promise<void> {
  try {
    const next = await invoke<boolean>('is_focus_mode_active');
    if (next !== cached) {
      cached = next;
      log.debug(cached ? 'Focus mode active' : 'Focus mode inactive');
    }
  } catch {
    cached = false;
  }
}

export function installFocusMode(): void {
  refresh();
  setInterval(refresh, POLL_INTERVAL);
}
