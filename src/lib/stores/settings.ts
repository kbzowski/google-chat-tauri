import { type Writable, writable } from 'svelte/store';
import { type AppSettings, getSettings, setSettings } from '../ipc';

const DEFAULT: AppSettings = {
  accountIndex: 0,
  autoCheckForUpdates: true,
  autoLaunchAtLogin: false,
  startHidden: false,
  hideMenuBar: true,
  disableSpellChecker: false,
  showOnMessage: true,
  theme: 'system',
  zoomLevel: 1.0,
  alwaysOnTop: false,
  notificationSound: true,
  globalShortcut: 'CmdOrCtrl+Shift+G',
  focusMode: false,
  focusModeDuration: 30,
  customCss: '',
  showUnreadInTitle: true,
  showUnreadInTray: true,
  minimizeToTray: false,
  externalLinksGuardEnabled: true,
  externalLinksGuardDisabledUntil: null,
};

export const settings: Writable<AppSettings> = writable({ ...DEFAULT });

let loaded = false;

export async function loadSettings(): Promise<void> {
  if (loaded) return;
  try {
    settings.set(await getSettings());
    loaded = true;
  } catch {
    // Fall back to defaults already initialised.
  }
}

export async function updateSetting<K extends keyof AppSettings>(
  key: K,
  value: AppSettings[K],
): Promise<void> {
  let current!: AppSettings;
  settings.update((s) => {
    current = { ...s, [key]: value };
    return current;
  });
  await setSettings(current);
}

export function _resetForTests(): void {
  loaded = false;
  settings.set({ ...DEFAULT });
}
