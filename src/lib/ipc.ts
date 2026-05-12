import { invoke } from '@tauri-apps/api/core';
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';

export type Theme = 'system' | 'light' | 'dark';

export interface AppSettings {
  autoCheckForUpdates: boolean;
  autoLaunchAtLogin: boolean;
  startHidden: boolean;
  hideMenuBar: boolean;
  disableSpellChecker: boolean;
  showOnMessage: boolean;
  theme: Theme;
  zoomLevel: number;
  alwaysOnTop: boolean;
  notificationSound: boolean;
  globalShortcut: string;
  focusMode: boolean;
  focusModeDuration: number;
  customCss: string;
  showUnreadInTitle: boolean;
  showUnreadInTray: boolean;
  externalLinksGuardEnabled: boolean;
  externalLinksGuardDisabledUntil: number | null;
}

export interface DebugInfo {
  appVersion: string;
  firefoxVersion: string;
  userAgent: string;
  buildDate: string;
  platform: string;
  webviewVersion: string;
}

export const getSettings = () => invoke<AppSettings>('get_settings');
export const setSettings = (settings: AppSettings) => invoke<void>('set_settings', { settings });
export const checkIfOnline = () => invoke<boolean>('check_if_online');
export const getDebugInfo = () => invoke<DebugInfo>('get_debug_info');

export type { UnlistenFn };
export { emit, listen };
