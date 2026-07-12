import { logger } from './lib/logger';

const log = logger('injection::auth-redirect');

// Hosts that are part of Google's sign-in flow (NOT the chat app itself). On these
// pages we keep popups / new-tab links inside the same webview so login completes
// in-app. WebView2 otherwise hands `window.open` and `target="_blank"` navigations
// to the OS browser, which has a separate session, so sign-in never lands here.
const AUTH_HOSTS = new Set([
  'accounts.youtube.com',
  'myaccount.google.com',
  'workspace.google.com',
  'gds.google.com',
  'challenges.google.com',
]);

// `accounts.google.com` plus locale ccTLDs (accounts.google.pl, ...) used by the
// cross-domain session flow.
const isAuthHost = (host: string): boolean =>
  AUTH_HOSTS.has(host) || host === 'accounts.google.com' || host.startsWith('accounts.google.');

export function installAuthPopupRedirect(): void {
  // Top frame only: chat pages (chat.google.com / mail.google.com) must keep their
  // default external-link behavior, and we don't touch Google's auth iframes.
  if (window.top !== window) return;
  if (!isAuthHost(location.hostname)) return;

  // 1) window.open -> navigate this frame instead of spawning a popup that escapes.
  const nativeOpen = window.open.bind(window);
  window.open = ((url?: string | URL, ...rest: unknown[]): Window | null => {
    if (url) {
      log.info('redirect window.open to top frame', { url: String(url) });
      location.assign(String(url));
      return null;
    }
    return nativeOpen(url as string, ...(rest as []));
  }) as typeof window.open;

  // 2) target="_blank" anchors -> same thing. Captured before the page's own handlers.
  document.addEventListener(
    'click',
    (event) => {
      const anchor = (event.target as Element | null)?.closest?.('a');
      if (anchor?.target !== '_blank') return;
      const href = anchor.getAttribute('href');
      if (!href) return;
      const url = new URL(href, location.href);
      if (url.protocol !== 'http:' && url.protocol !== 'https:') return;
      event.preventDefault();
      log.info('redirect _blank anchor to top frame', { url: url.href });
      location.assign(url.href);
    },
    true,
  );

  log.debug('auth popup redirect installed', { host: location.hostname });
}
