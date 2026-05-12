import { get } from 'svelte/store';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('../../ipc', () => ({
  getSettings: vi.fn(),
  setSettings: vi.fn(() => Promise.resolve()),
}));

import { _resetForTests, settings, updateSetting } from '../settings';

describe('updateSetting', () => {
  beforeEach(() => {
    _resetForTests();
    vi.clearAllMocks();
  });

  it('mutates the store and calls setSettings with merged value', async () => {
    const { setSettings } = await import('../../ipc');
    await updateSetting('alwaysOnTop', true);
    expect(get(settings).alwaysOnTop).toBe(true);
    expect(setSettings).toHaveBeenCalledWith(expect.objectContaining({ alwaysOnTop: true }));
  });

  it('preserves other fields when one changes', async () => {
    await updateSetting('zoomLevel', 1.5);
    expect(get(settings).zoomLevel).toBe(1.5);
    expect(get(settings).autoCheckForUpdates).toBe(true);
  });
});
