import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  emit: vi.fn(() => Promise.resolve()),
}));

import { emit } from '@tauri-apps/api/event';
import { buildPatched, EVENT_MESSAGE } from '../notification-patch';

describe('buildPatched', () => {
  beforeEach(() => {
    vi.mocked(emit).mockClear();
  });

  it('forwards title and body to the native toast instead of constructing one', () => {
    const instance = new (buildPatched())('Daria Kordys', { body: 'cos' } as NotificationOptions);
    expect(emit).toHaveBeenCalledWith(EVENT_MESSAGE, { title: 'Daria Kordys', body: 'cos' });
    // Returned object is the inert stub, not a real Notification.
    expect((instance as unknown as { title: string }).title).toBe('');
  });

  it('defaults missing body to an empty string', () => {
    new (buildPatched())('Sender');
    expect(emit).toHaveBeenCalledWith(EVENT_MESSAGE, { title: 'Sender', body: '' });
  });

  it('does not emit while suppressed (focus mode)', () => {
    new (buildPatched(() => true))('Sender', { body: 'hi' } as NotificationOptions);
    expect(emit).not.toHaveBeenCalled();
  });

  it('reports granted permission even though WebView2 never granted it', () => {
    expect(buildPatched().permission).toBe('granted');
  });

  it('resolves requestPermission to granted', async () => {
    await expect(buildPatched().requestPermission()).resolves.toBe('granted');
  });
});
