import { listen } from '@tauri-apps/api/event';
import { logger } from './lib/logger';

const log = logger('injection::search');
export const EVENT = 'search-shortcut';

function isVisible(el: HTMLElement): boolean {
  return !!(el.offsetWidth || el.offsetHeight || el.getClientRects().length);
}

export function focusSearchInput(): boolean {
  const input = document.querySelector<HTMLInputElement>('input[name="q"]');
  if (input && isVisible(input)) {
    input.focus();
    return true;
  }
  return false;
}

export function installSearchFocus(): void {
  listen(EVENT, () => {
    const focused = focusSearchInput();
    log.debug(focused ? 'Search focused' : 'Search element not found');
  }).catch(() => {});
}
