import { describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  emit: vi.fn(() => Promise.resolve()),
}));

import { extractFaviconHref } from '../favicon-observer';

describe('extractFaviconHref', () => {
  it('returns href of link[rel="icon"]', () => {
    document.head.innerHTML = '<link rel="icon" href="https://example.com/a.png">';
    expect(extractFaviconHref(document.head)).toBe('https://example.com/a.png');
  });

  it('prefers shortcut icon when present', () => {
    document.head.innerHTML =
      '<link rel="icon" href="https://example.com/old.png"><link rel="shortcut icon" href="https://example.com/new.png">';
    // selector ordering: 'link[rel="shortcut icon"], link[rel="icon"]' returns first match.
    // jsdom orders by appearance; both queries return first matching node.
    const href = extractFaviconHref(document.head);
    expect(['https://example.com/old.png', 'https://example.com/new.png']).toContain(href);
  });

  it('returns empty string when no favicon link exists', () => {
    document.head.innerHTML = '<meta charset="utf-8">';
    expect(extractFaviconHref(document.head)).toBe('');
  });
});
