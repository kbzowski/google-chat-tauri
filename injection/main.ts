import { installErrorCapture } from './error-capture';
import { installNotificationPatch } from './notification-patch';

declare global {
  interface Window {
    __googleChatTauriInjected?: boolean;
  }
}

if (!window.__googleChatTauriInjected) {
  window.__googleChatTauriInjected = true;
  installErrorCapture();
  installNotificationPatch();
}
