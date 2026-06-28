import { emit } from '@tauri-apps/api/event';
import { extractFaviconHref } from './favicon-observer';
import { debounce } from './lib/debounce';
import { logger } from './lib/logger';

const log = logger('injection::unread');
export const EVENT = 'unread-count';
const PRIMARY = '.XS > span > .XU';
const FALLBACK = 'div[data-tooltip="Chat"][role="group"], div[data-tooltip="Spaces"][role="group"]';
const TITLE_REGEX = /^\((\d+)\)/;
const FAVICON_NOTIF_MARKER = 'new_notif';

export function countFromTitle(title: string): number {
  const match = TITLE_REGEX.exec(title);
  return match ? Number(match[1]) : 0;
}

export function countFromPrimary(root: ParentNode): number {
  let counter = 0;
  for (const el of root.querySelectorAll<HTMLElement>(PRIMARY)) {
    counter += Number(el.textContent ?? 0);
  }
  return counter;
}

export function countFromFallback(root: ParentNode): number {
  let counter = 0;
  for (const group of root.querySelectorAll<HTMLElement>(FALLBACK)) {
    const heading = group.querySelector('span[role="heading"]');
    const sibling = heading?.nextElementSibling;
    if (sibling?.textContent) counter += Number(sibling.textContent);
  }
  return counter;
}

export function countFromFavicon(href: string): number {
  return href.includes(FAVICON_NOTIF_MARKER) ? 1 : 0;
}

export function computeUnreadCount(): number {
  const primary = countFromPrimary(document.body);
  if (primary > 0) return primary;
  const fallback = countFromFallback(document.body);
  if (fallback > 0) return fallback;
  const fromTitle = countFromTitle(document.title);
  if (fromTitle > 0) return fromTitle;
  return countFromFavicon(extractFaviconHref(document.head));
}

export function installUnreadCounter(): void {
  let previous = -1;
  const check = debounce(() => {
    const count = computeUnreadCount();
    if (count !== previous) {
      previous = count;
      log.debug('Unread count changed', { count });
      emit(EVENT, { count }).catch(() => {});
    }
  }, 250);
  const observer = new MutationObserver(check);
  observer.observe(document.body, {
    childList: true,
    subtree: true,
    characterData: true,
  });
  const titleEl = document.querySelector('title');
  if (titleEl) {
    new MutationObserver(check).observe(titleEl, { childList: true });
  }
  new MutationObserver(check).observe(document.head, {
    childList: true,
    subtree: true,
    attributes: true,
  });
  check();
}
