import { describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/event', () => ({
  emit: vi.fn(() => Promise.resolve()),
}));

import { buildPatched, EVENT_CLICKED } from '../notification-patch';

class FakeNotification {
  static permission: NotificationPermission = 'granted';
  static requestPermission = vi.fn(() => Promise.resolve('granted' as NotificationPermission));
  title: string;
  options: NotificationOptions;
  close = vi.fn();
  private listeners: Record<string, (() => void)[]> = {};
  constructor(title: string, options: NotificationOptions = {}) {
    this.title = title;
    this.options = options;
  }
  addEventListener(name: string, fn: () => void) {
    const arr = this.listeners[name] ?? [];
    arr.push(fn);
    this.listeners[name] = arr;
  }
  fire(name: string) {
    for (const fn of this.listeners[name] ?? []) fn();
  }
}

describe('buildPatched', () => {
  it('forces requireInteraction and silent options', () => {
    const Patched = buildPatched(FakeNotification as unknown as typeof Notification);
    const instance = new Patched('hi', { body: 'msg', silent: true } as NotificationOptions);
    const fake = instance as unknown as FakeNotification;
    expect(fake.options.requireInteraction).toBe(true);
    expect(fake.options.silent).toBe(false);
    expect(fake.options.body).toBe('msg');
  });

  it('emits EVENT_CLICKED on click and closes notification', async () => {
    const { emit } = await import('@tauri-apps/api/event');
    const Patched = buildPatched(FakeNotification as unknown as typeof Notification);
    const instance = new Patched('hi');
    const fake = instance as unknown as FakeNotification;
    fake.fire('click');
    expect(emit).toHaveBeenCalledWith(EVENT_CLICKED);
    expect(fake.close).toHaveBeenCalled();
  });

  it('exposes permission and requestPermission from original', () => {
    const Patched = buildPatched(FakeNotification as unknown as typeof Notification);
    expect(Patched.permission).toBe('granted');
    expect(typeof Patched.requestPermission).toBe('function');
  });
});
