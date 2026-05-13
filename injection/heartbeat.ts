import { emit } from '@tauri-apps/api/event';

const INTERVAL = 5000;
export const EVENT = 'webview-heartbeat';

export function installHeartbeat(): void {
  setInterval(() => {
    emit(EVENT).catch(() => {});
  }, INTERVAL);
}
