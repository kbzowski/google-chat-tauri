import { installErrorCapture } from './error-capture';

declare global {
  interface Window {
    __googleChatTauriInjected?: boolean;
  }
}

if (!window.__googleChatTauriInjected) {
  window.__googleChatTauriInjected = true;
  installErrorCapture();
}
