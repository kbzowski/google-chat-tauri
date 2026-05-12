import { emit } from '@tauri-apps/api/event';
import { debounce } from './lib/debounce';
import { logger } from './lib/logger';

const log = logger('injection::favicon');
export const EVENT = 'favicon-changed';
const SELECTORS = 'link[rel="shortcut icon"], link[rel="icon"]';

export function extractFaviconHref(head: ParentNode): string {
  const link = head.querySelector<HTMLLinkElement>(SELECTORS);
  return link?.href ?? '';
}

export function installFaviconObserver(): void {
  let previous = '';
  const check = debounce(() => {
    const href = extractFaviconHref(document.head);
    if (href !== previous) {
      previous = href;
      log.debug('Favicon changed', { href });
      emit(EVENT, { href }).catch(() => {});
    }
  }, 100);
  const observer = new MutationObserver(check);
  observer.observe(document.head, { childList: true, subtree: true, attributes: true });
  check();
}
