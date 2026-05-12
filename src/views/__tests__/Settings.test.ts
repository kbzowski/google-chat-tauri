import { fireEvent, render } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('../../lib/ipc', () => ({
  getSettings: vi.fn(() =>
    Promise.resolve({
      autoCheckForUpdates: true,
      autoLaunchAtLogin: false,
      startHidden: false,
      hideMenuBar: false,
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
      externalLinksGuardEnabled: true,
      externalLinksGuardDisabledUntil: null,
    }),
  ),
  setSettings: vi.fn(() => Promise.resolve()),
}));

import { _resetForTests } from '../../lib/stores/settings';
import Settings from '../Settings.svelte';

describe('Settings view', () => {
  beforeEach(() => {
    _resetForTests();
    vi.clearAllMocks();
  });

  it('renders Startup/Notifications/Appearance/Advanced sections', () => {
    const { getByText } = render(Settings);
    expect(getByText('Startup')).toBeInTheDocument();
    expect(getByText('Notifications')).toBeInTheDocument();
    expect(getByText('Appearance')).toBeInTheDocument();
    expect(getByText('Advanced')).toBeInTheDocument();
  });

  it('toggles a checkbox and calls setSettings', async () => {
    const { setSettings } = await import('../../lib/ipc');
    const { getByLabelText } = render(Settings);
    const aot = getByLabelText(/always on top/i) as HTMLInputElement;
    await fireEvent.click(aot);
    expect(setSettings).toHaveBeenCalledWith(expect.objectContaining({ alwaysOnTop: true }));
  });
});
