import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { logger } from './lib/logger';

const log = logger('injection::zoom');
const ZOOM_MIN = 0.5;
const ZOOM_MAX = 3.0;
const ZOOM_STEP = 0.1;
export const EVENT_APPLY = 'apply-zoom';

let current = 1.0;

export function clamp(value: number, min = ZOOM_MIN, max = ZOOM_MAX): number {
  return Math.max(min, Math.min(max, value));
}

export function applyZoom(level: number): void {
  current = clamp(level);
  (document.body.style as CSSStyleDeclaration & { zoom: string }).zoom = String(current);
}

async function persist(level: number): Promise<void> {
  try {
    await invoke('set_zoom_level', { level });
  } catch (err) {
    log.warn('Failed to persist zoom', { err: String(err) });
  }
}

async function loadInitial(): Promise<number> {
  try {
    return await invoke<number>('get_zoom_level');
  } catch {
    return 1.0;
  }
}

export function installZoom(): void {
  loadInitial().then((saved) => applyZoom(saved));
  listen<number>(EVENT_APPLY, ({ payload }) => applyZoom(payload)).catch(() => {});
  document.addEventListener('keydown', (e) => {
    if (!(e.ctrlKey || e.metaKey)) return;
    if (e.key === '=' || e.key === '+') {
      e.preventDefault();
      applyZoom(current + ZOOM_STEP);
      persist(current);
    } else if (e.key === '-') {
      e.preventDefault();
      applyZoom(current - ZOOM_STEP);
      persist(current);
    } else if (e.key === '0') {
      e.preventDefault();
      applyZoom(1.0);
      persist(current);
    }
  });
}
