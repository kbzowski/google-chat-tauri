import { installErrorCapture } from './error-capture';
import { installFaviconObserver } from './favicon-observer';
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

  const initDom = () => {
    installFaviconObserver();
  };
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initDom);
  } else {
    initDom();
  }
}
