import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(() => Promise.resolve(1.0)),
}));

import { applyZoom, clamp } from '../zoom';

describe('clamp', () => {
  it('clamps below minimum', () => {
    expect(clamp(0.1)).toBe(0.5);
  });
  it('clamps above maximum', () => {
    expect(clamp(10)).toBe(3.0);
  });
  it('returns value when within range', () => {
    expect(clamp(1.5)).toBe(1.5);
  });
});

describe('applyZoom', () => {
  beforeEach(() => {
    (document.body.style as unknown as { zoom: string }).zoom = '';
  });
  it('sets document.body.style.zoom', () => {
    applyZoom(1.5);
    expect((document.body.style as unknown as { zoom: string }).zoom).toBe('1.5');
  });
  it('respects clamp bounds', () => {
    applyZoom(99);
    expect((document.body.style as unknown as { zoom: string }).zoom).toBe('3');
  });
});
