import { listen } from '@tauri-apps/api/event';
import { logger } from './lib/logger';

const log = logger('injection::notification');
export const EVENT_ACTIVATED = 'notification-activated';

// Conversation rows in the list carry a stable `data-group-id` ("dm/…",
// "space/…"). The list is ordered most-recent-first, so the first unread row is
// the conversation the newest message belongs to.
const ITEM_SELECTOR = 'span[role="listitem"][data-group-id]';
// Per-conversation unread badge — the same element the unread counter reads.
const UNREAD_BADGE = '.XU';
// Obfuscated "unread" style class observed on unread rows; a fallback for when
// the badge is absent (e.g. muted/rolled-up unreads).
const UNREAD_CLASS = 'H7du2';

function isConversation(item: HTMLElement): boolean {
  const group = item.getAttribute('data-group-id') ?? '';
  return group.startsWith('dm/') || group.startsWith('space/');
}

export function isUnread(item: HTMLElement): boolean {
  return item.querySelector(UNREAD_BADGE) !== null || item.classList.contains(UNREAD_CLASS);
}

export function findMostRecentUnread(root: ParentNode = document): HTMLElement | null {
  for (const item of root.querySelectorAll<HTMLElement>(ITEM_SELECTOR)) {
    if (isConversation(item) && isUnread(item)) return item;
  }
  return null;
}

// Chat's list rows are Closure `jsaction` targets that react to a full pointer
// gesture, so a bare `.click()` can be ignored — dispatch the sequence.
function simulateClick(el: HTMLElement): void {
  const opts = { bubbles: true, cancelable: true, view: window };
  el.dispatchEvent(new PointerEvent('pointerdown', opts));
  el.dispatchEvent(new MouseEvent('mousedown', opts));
  el.dispatchEvent(new PointerEvent('pointerup', opts));
  el.dispatchEvent(new MouseEvent('mouseup', opts));
  el.dispatchEvent(new MouseEvent('click', opts));
}

export function openMostRecentUnread(): boolean {
  const item = findMostRecentUnread();
  if (!item) {
    log.debug('No unread conversation to open on activation');
    return false;
  }
  const group = item.getAttribute('data-group-id');
  log.debug('Opening unread conversation on activation', { group });
  item.scrollIntoView({ block: 'nearest' });
  simulateClick(item);
  return true;
}

export function installNotificationActivation(): void {
  listen(EVENT_ACTIVATED, () => {
    openMostRecentUnread();
  }).catch(() => {});
}
