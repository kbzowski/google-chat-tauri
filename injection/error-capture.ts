import { logger } from './lib/logger';

const log = logger('injection::uncaught');

export function installErrorCapture(): void {
  window.addEventListener('error', (event) => {
    log.error('Uncaught JavaScript error', {
      message: event.message,
      filename: event.filename?.split('/').pop() ?? 'unknown',
      lineno: event.lineno,
      colno: event.colno,
    });
  });
  window.addEventListener('unhandledrejection', (event) => {
    log.error('Unhandled Promise rejection', {
      reason: String(event.reason).substring(0, 200),
    });
  });
}
