import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { logger } from './lib/logger';

const log = logger('injection::custom-css');
const STYLE_ID = 'tauri-custom-css';
export const EVENT_APPLY = 'apply-custom-css';

export function applyCustomCss(css: string): void {
  const existing = document.getElementById(STYLE_ID);
  existing?.remove();
  if (!css.trim()) return;
  const style = document.createElement('style');
  style.id = STYLE_ID;
  style.textContent = css;
  document.head.appendChild(style);
}

export function installCustomCss(): void {
  invoke<string>('get_custom_css')
    .then((css) => applyCustomCss(css))
    .catch(() => {});
  listen<{ css: string }>(EVENT_APPLY, ({ payload }) => {
    log.debug('Applying custom CSS', { length: payload.css.length });
    applyCustomCss(payload.css);
  }).catch(() => {});
}
