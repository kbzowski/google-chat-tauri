import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

import { findMostRecentUnread, isUnread } from '../notification-activation';

function row(group: string, opts: { unreadClass?: boolean; badge?: boolean } = {}): HTMLElement {
  const el = document.createElement('span');
  el.setAttribute('role', 'listitem');
  el.setAttribute('data-group-id', group);
  if (opts.unreadClass) el.classList.add('H7du2');
  if (opts.badge) {
    const badge = document.createElement('span');
    badge.className = 'XU';
    badge.textContent = '1';
    el.appendChild(badge);
  }
  return el;
}

describe('isUnread', () => {
  it('detects the unread style class', () => {
    expect(isUnread(row('dm/a', { unreadClass: true }))).toBe(true);
  });

  it('detects the per-conversation unread badge', () => {
    expect(isUnread(row('dm/a', { badge: true }))).toBe(true);
  });

  it('treats a plain row as read', () => {
    expect(isUnread(row('dm/a'))).toBe(false);
  });
});

describe('findMostRecentUnread', () => {
  beforeEach(() => {
    document.body.innerHTML = '';
  });

  it('returns the first unread conversation in list order', () => {
    const root = document.createElement('div');
    root.append(
      row('dm/read-1'),
      row('dm/unread-top', { unreadClass: true }),
      row('dm/unread-lower', { badge: true }),
    );
    expect(findMostRecentUnread(root)?.getAttribute('data-group-id')).toBe('dm/unread-top');
  });

  it('finds unread spaces (group conversations), not only DMs', () => {
    const root = document.createElement('div');
    root.append(row('dm/read'), row('space/AAQAxyz', { unreadClass: true }));
    expect(findMostRecentUnread(root)?.getAttribute('data-group-id')).toBe('space/AAQAxyz');
  });

  it('ignores non-conversation rows such as pinned-item', () => {
    const root = document.createElement('div');
    const pinned = document.createElement('div');
    pinned.setAttribute('data-group-id', 'pinned-item');
    pinned.classList.add('H7du2');
    root.append(pinned, row('dm/only', { badge: true }));
    expect(findMostRecentUnread(root)?.getAttribute('data-group-id')).toBe('dm/only');
  });

  it('returns null when every conversation is read', () => {
    const root = document.createElement('div');
    root.append(row('dm/a'), row('space/b'));
    expect(findMostRecentUnread(root)).toBeNull();
  });
});
