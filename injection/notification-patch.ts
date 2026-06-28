import { emit } from '@tauri-apps/api/event';
import { isFocusModeActive } from './focus-mode';
import { logger } from './lib/logger';

const log = logger('injection::notification');
export const EVENT_MESSAGE = 'notification-message';

class SuppressedNotification extends EventTarget {
  close(): void {}
  // Mirror Notification interface stub - properties default to empty strings.
  title = '';
  body = '';
  tag = '';
}

export interface MessagePayload {
  title: string;
  body: string;
}

export function buildPatched(isSuppressed: () => boolean = isFocusModeActive): typeof Notification {
  function Patched(title: string, options: NotificationOptions = {}): Notification {
    if (!isSuppressed()) {
      const payload: MessagePayload = { title, body: options.body ?? '' };
      emit(EVENT_MESSAGE, payload).catch(() => {});
    }
    // Never construct a real browser notification: WebView2 does not surface web
    // notifications to the OS, and the native toast (driven by EVENT_MESSAGE) owns
    // display and click handling.
    return new SuppressedNotification() as unknown as Notification;
  }
  // Google Chat only constructs notifications when it believes permission is
  // granted. WebView2 never grants it (stays "default"), so report granted and
  // resolve requestPermission accordingly to unlock the constructor above.
  Patched.requestPermission = ((cb?: NotificationPermissionCallback) => {
    cb?.('granted');
    return Promise.resolve('granted' as NotificationPermission);
  }) as typeof Notification.requestPermission;
  Object.defineProperty(Patched, 'permission', {
    configurable: true,
    enumerable: true,
    get: () => 'granted',
    set: () => {
      // Permission is owned by the browser; silently ignore writes.
    },
  });
  return Patched as unknown as typeof Notification;
}

export function installNotificationPatch(): void {
  const Original = window.Notification;
  if (!Original) {
    log.warn('window.Notification is not available, skipping patch');
    return;
  }
  (window as unknown as { Notification: typeof Notification }).Notification =
    buildPatched(Original);
  log.debug('Notification API patched');
}
