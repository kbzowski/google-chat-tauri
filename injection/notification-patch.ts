import { emit } from '@tauri-apps/api/event';
import { logger } from './lib/logger';

const log = logger('injection::notification');
export const EVENT_CLICKED = 'notification-clicked';

export function buildPatched(Original: typeof Notification): typeof Notification {
  function Patched(title: string, options: NotificationOptions = {}): Notification {
    const instance = new Original(title, {
      ...options,
      requireInteraction: true,
      silent: false,
    });
    instance.addEventListener('click', () => {
      emit(EVENT_CLICKED).catch(() => {});
      instance.close();
    });
    return instance;
  }
  Patched.requestPermission = Original.requestPermission.bind(Original);
  // Mirror Original.permission as a writable property. Google Chat reassigns
  // window.Notification.permission and a getter-only property would throw.
  Object.defineProperty(Patched, 'permission', {
    configurable: true,
    enumerable: true,
    get: () => Original.permission,
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
