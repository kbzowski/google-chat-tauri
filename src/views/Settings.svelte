<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMount } from 'svelte';
import Toggle from '../lib/components/Toggle.svelte';
import type { Theme } from '../lib/ipc';
import { loadSettings, settings, updateSetting } from '../lib/stores/settings';

let current = $state($settings);
let unsubscribe: () => void = () => {};

onMount(() => {
  unsubscribe = settings.subscribe((value) => {
    current = value;
  });
  loadSettings();
  return () => unsubscribe();
});

function set<K extends keyof typeof current>(key: K, value: (typeof current)[K]) {
  updateSetting(key, value);
}

async function toggleFocusMode(active: boolean) {
  if (active) {
    await invoke('enable_focus_mode', { durationMinutes: current.focusModeDuration });
  } else {
    await invoke('disable_focus_mode');
  }
  set('focusMode', active);
}
</script>

<main>
  <h1>Settings</h1>

  <section>
    <h2>Startup</h2>
    <label class="row">
      <span>
        <span class="label">Google account index</span>
        <span class="description">URL becomes mail.google.com/chat/u/&lt;index&gt;. Restart required.</span>
      </span>
      <input
        type="number"
        min="0"
        max="9"
        value={current.accountIndex}
        onchange={(e) =>
          set('accountIndex', Number.parseInt((e.currentTarget as HTMLInputElement).value, 10))}
      />
    </label>
    <Toggle
      label="Auto Launch at Login"
      description="Start google-chat-tauri when you sign in to Windows"
      checked={current.autoLaunchAtLogin}
      onchange={(v) => set('autoLaunchAtLogin', v)}
    />
    <Toggle
      label="Start Hidden"
      description="Launch minimized to the system tray"
      checked={current.startHidden}
      onchange={(v) => set('startHidden', v)}
    />
    <Toggle
      label="Auto check for Updates"
      checked={current.autoCheckForUpdates}
      onchange={(v) => set('autoCheckForUpdates', v)}
    />
    <Toggle
      label="Minimize to tray"
      description="Hide the window when minimized instead of leaving it in the taskbar"
      checked={current.minimizeToTray}
      onchange={(v) => set('minimizeToTray', v)}
    />
  </section>

  <section>
    <h2>Notifications</h2>
    <Toggle
      label="Show window on new message"
      checked={current.showOnMessage}
      onchange={(v) => set('showOnMessage', v)}
    />
    <Toggle
      label="Notification sound"
      checked={current.notificationSound}
      onchange={(v) => set('notificationSound', v)}
    />
    <Toggle
      label="Disable spell checker"
      checked={current.disableSpellChecker}
      onchange={(v) => set('disableSpellChecker', v)}
    />
    <Toggle
      label="Show unread count in window title"
      checked={current.showUnreadInTitle}
      onchange={(v) => set('showUnreadInTitle', v)}
    />
    <Toggle
      label="Show unread count in tray icon"
      checked={current.showUnreadInTray}
      onchange={(v) => set('showUnreadInTray', v)}
    />
  </section>

  <section>
    <h2>Appearance</h2>
    <label class="row">
      <span>Theme</span>
      <select
        value={current.theme}
        onchange={(e) => set('theme', (e.currentTarget as HTMLSelectElement).value as Theme)}
      >
        <option value="system">System</option>
        <option value="light">Light</option>
        <option value="dark">Dark</option>
      </select>
    </label>
    <label class="row">
      <span>Zoom level</span>
      <input
        type="number"
        min="0.5"
        max="3"
        step="0.1"
        value={current.zoomLevel}
        onchange={(e) =>
          set('zoomLevel', Number.parseFloat((e.currentTarget as HTMLInputElement).value))}
      />
    </label>
    <Toggle
      label="Always on Top"
      checked={current.alwaysOnTop}
      onchange={(v) => set('alwaysOnTop', v)}
    />
    <Toggle
      label="Hide Menu Bar"
      description="Press Alt to reveal the menu temporarily"
      checked={current.hideMenuBar}
      onchange={(v) => set('hideMenuBar', v)}
    />
  </section>

  <section>
    <h2>Focus Mode</h2>
    <Toggle
      label="Active"
      description="Suppresses notifications for the configured duration"
      checked={current.focusMode}
      onchange={toggleFocusMode}
    />
    <label class="row">
      <span>Duration (minutes)</span>
      <select
        value={String(current.focusModeDuration)}
        onchange={(e) =>
          set(
            'focusModeDuration',
            Number.parseInt((e.currentTarget as HTMLSelectElement).value, 10),
          )}
      >
        <option value="5">5</option>
        <option value="15">15</option>
        <option value="25">25</option>
        <option value="60">60</option>
        <option value="120">120</option>
      </select>
    </label>
  </section>

  <section>
    <h2>Advanced</h2>
    <label class="row">
      <span>Global shortcut</span>
      <input
        type="text"
        value={current.globalShortcut}
        onchange={(e) => set('globalShortcut', (e.currentTarget as HTMLInputElement).value)}
      />
    </label>
    <label class="row column">
      <span>Custom CSS (applied inside Google Chat)</span>
      <textarea
        rows="6"
        value={current.customCss}
        onchange={(e) => set('customCss', (e.currentTarget as HTMLTextAreaElement).value)}
      ></textarea>
    </label>
  </section>
</main>

<style>
  main {
    padding: 1.5rem 2rem;
    max-width: 640px;
  }
  h1 {
    margin: 0 0 1rem;
    font-size: 1.4rem;
  }
  h2 {
    margin: 1.5rem 0 0.5rem;
    font-size: 1rem;
    text-transform: uppercase;
    color: var(--muted);
    letter-spacing: 0.04em;
  }
  section {
    border-top: 1px solid var(--border);
    padding-top: 0.5rem;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.5rem 0;
  }
  .row.column {
    flex-direction: column;
    align-items: stretch;
  }
  .row.column textarea {
    width: 100%;
    font-family: 'Cascadia Code', Consolas, monospace;
  }
</style>
