import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(() => Promise.resolve('')),
}));

import { applyCustomCss } from '../custom-css';

const STYLE_ID = 'tauri-custom-css';

describe('applyCustomCss', () => {
  beforeEach(() => {
    document.head.innerHTML = '';
  });

  it('appends a <style> with given content', () => {
    applyCustomCss('body { background: red; }');
    const style = document.getElementById(STYLE_ID);
    expect(style).not.toBeNull();
    expect(style?.tagName).toBe('STYLE');
    expect(style?.textContent).toBe('body { background: red; }');
  });

  it('replaces existing style with new content', () => {
    applyCustomCss('a { color: red; }');
    applyCustomCss('a { color: blue; }');
    const styles = document.querySelectorAll(`#${STYLE_ID}`);
    expect(styles).toHaveLength(1);
    expect(styles[0].textContent).toBe('a { color: blue; }');
  });

  it('removes style when given empty string', () => {
    applyCustomCss('body { color: red; }');
    applyCustomCss('');
    expect(document.getElementById(STYLE_ID)).toBeNull();
  });

  it('removes style when given whitespace-only string', () => {
    applyCustomCss('body { color: red; }');
    applyCustomCss('   \n  ');
    expect(document.getElementById(STYLE_ID)).toBeNull();
  });
});
