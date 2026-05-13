import { installCustomCss } from './custom-css';
import { installErrorCapture } from './error-capture';
import { installFaviconObserver } from './favicon-observer';
import { installFocusMode } from './focus-mode';
import { installHeartbeat } from './heartbeat';
import { installNotificationPatch } from './notification-patch';
import { installSearchFocus } from './search-focus';
import { installUnreadCounter } from './unread-counter';
import { installZoom } from './zoom';

declare global {
  interface Window {
    __googleChatTauriInjected?: boolean;
  }
}

if (!window.__googleChatTauriInjected) {
  window.__googleChatTauriInjected = true;
  installErrorCapture();
  installFocusMode();
  installNotificationPatch();

  const initDom = () => {
    installFaviconObserver();
    installUnreadCounter();
    installSearchFocus();
    installZoom();
    installCustomCss();
    installHeartbeat();
  };
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initDom);
  } else {
    initDom();
  }
}
