import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  emit: vi.fn(() => Promise.resolve()),
}));

import {
  countFromFallback,
  countFromFavicon,
  countFromPrimary,
  countFromTitle,
} from '../unread-counter';

const FAVICON_BASE = 'https://www.gstatic.com/dynamite/images/favicons_20260602';

describe('countFromFavicon', () => {
  it('counts the dot favicon variant as unread', () => {
    expect(countFromFavicon(`${FAVICON_BASE}/chat_2026_logo_favicon_dot_64px.png`)).toBe(1);
  });

  it('treats the no-dot variant as read', () => {
    expect(countFromFavicon(`${FAVICON_BASE}/chat_2026_logo_favicon_no_dot_64px.png`)).toBe(0);
  });

  it('returns null for an absent favicon so the caller holds the previous count', () => {
    expect(countFromFavicon('')).toBeNull();
  });
});

describe('countFromTitle', () => {
  it('extracts leading parenthesised number', () => {
    expect(countFromTitle('(5) Google Chat')).toBe(5);
    expect(countFromTitle('(99) Chat')).toBe(99);
  });

  it('returns zero when no leading number', () => {
    expect(countFromTitle('Google Chat')).toBe(0);
    expect(countFromTitle('Google (Chat)')).toBe(0);
    expect(countFromTitle('')).toBe(0);
  });
});

describe('countFromPrimary', () => {
  beforeEach(() => {
    document.body.innerHTML = '';
  });

  it('sums numeric text content under primary selector', () => {
    document.body.innerHTML = `
      <div class="XS"><span><span class="XU">3</span></span></div>
      <div class="XS"><span><span class="XU">7</span></span></div>
    `;
    expect(countFromPrimary(document.body)).toBe(10);
  });

  it('returns zero when selector matches nothing', () => {
    document.body.innerHTML = '<div></div>';
    expect(countFromPrimary(document.body)).toBe(0);
  });
});

describe('countFromFallback', () => {
  beforeEach(() => {
    document.body.innerHTML = '';
  });

  it('reads sibling after heading inside tooltip group', () => {
    document.body.innerHTML = `
      <div data-tooltip="Chat" role="group">
        <span role="heading">Chats</span>
        <span>4</span>
      </div>
    `;
    expect(countFromFallback(document.body)).toBe(4);
  });

  it('returns zero when no group element present', () => {
    document.body.innerHTML = '<div role="group">no tooltip</div>';
    expect(countFromFallback(document.body)).toBe(0);
  });
});
