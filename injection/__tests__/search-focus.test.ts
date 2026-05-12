import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

import { focusSearchInput } from '../search-focus';

describe('focusSearchInput', () => {
  beforeEach(() => {
    document.body.innerHTML = '';
  });

  it('focuses input[name="q"] when visible', () => {
    document.body.innerHTML = '<input name="q" />';
    const input = document.querySelector<HTMLInputElement>('input[name="q"]');
    // jsdom layout returns 0 for offset*, override getClientRects to fake visibility.
    if (input) {
      input.getClientRects = () =>
        [
          { width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0 },
        ] as unknown as DOMRectList;
    }
    expect(focusSearchInput()).toBe(true);
    expect(document.activeElement).toBe(input);
  });

  it('returns false when no input exists', () => {
    expect(focusSearchInput()).toBe(false);
  });

  it('returns false when input is invisible', () => {
    document.body.innerHTML = '<input name="q" />';
    const input = document.querySelector<HTMLInputElement>('input[name="q"]');
    if (input) {
      Object.defineProperty(input, 'offsetWidth', { value: 0 });
      Object.defineProperty(input, 'offsetHeight', { value: 0 });
      input.getClientRects = () => [] as unknown as DOMRectList;
    }
    expect(focusSearchInput()).toBe(false);
  });
});
