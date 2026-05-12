import { describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  emit: vi.fn(() => Promise.resolve()),
}));

import { EVENT_LOG, logger, makeEntry } from '../lib/logger';

describe('makeEntry', () => {
  it('returns a well-formed entry', () => {
    const entry = makeEntry('debug', 'mod::test', 'hi', { x: 1 });
    expect(entry.level).toBe('debug');
    expect(entry.target).toBe('mod::test');
    expect(entry.message).toBe('hi');
    expect(entry.fields).toEqual({ x: 1 });
    expect(entry.timestamp).toMatch(/^\d{4}-\d{2}-\d{2}T/);
  });
});

describe('logger', () => {
  it('emits each level under EVENT_LOG', async () => {
    const { emit } = await import('@tauri-apps/api/event');
    const log = logger('mod::sample');
    await log.info('hello', { a: 'b' });
    expect(emit).toHaveBeenCalledWith(
      EVENT_LOG,
      expect.objectContaining({
        level: 'info',
        target: 'mod::sample',
        message: 'hello',
        fields: { a: 'b' },
      }),
    );
  });
});
