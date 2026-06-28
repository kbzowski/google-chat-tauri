<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
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

async function set<K extends keyof typeof current>(key: K, value: (typeof current)[K]) {
  await updateSetting(key, value);
  if (key === 'theme') await emit('apply-theme', value);
  else if (key === 'zoomLevel') await emit('apply-zoom', value);
  else if (key === 'customCss') await emit('apply-custom-css', { css: value });
  else if (key === 'alwaysOnTop') await emit('apply-always-on-top', value);
  else if (key === 'globalShortcut') await emit('apply-shortcut', value);
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
  <header>
    <h1>Settings</h1>
    <p class="subtitle">Configure how google-chat-tauri behaves on this machine.</p>
  </header>

  <section class="card">
    <header class="card-header">
      <h2>Startup</h2>
      <p>Behaviour when launching the app and signing in.</p>
    </header>
    <div class="card-body">
      <label class="field">
        <span class="meta">
          <span class="label">Google account index</span>
          <span class="description">
            URL becomes <code>mail.google.com/chat/u/&lt;index&gt;</code>. Restart required.
          </span>
        </span>
        <input
          class="control control-narrow"
          type="number"
          min="0"
          max="9"
          value={current.accountIndex}
          onchange={(e) =>
            set('accountIndex', Number.parseInt((e.currentTarget as HTMLInputElement).value, 10))}
        />
      </label>
      <Toggle
        label="Auto launch at login"
        description="Start google-chat-tauri when you sign in to Windows"
        checked={current.autoLaunchAtLogin}
        onchange={(v) => set('autoLaunchAtLogin', v)}
      />
      <Toggle
        label="Start hidden"
        description="Launch minimized to the system tray"
        checked={current.startHidden}
        onchange={(v) => set('startHidden', v)}
      />
      <Toggle
        label="Auto check for updates"
        description="Look for a new release 5 seconds after startup"
        checked={current.autoCheckForUpdates}
        onchange={(v) => set('autoCheckForUpdates', v)}
      />
      <Toggle
        label="Minimize to tray"
        description="Hide the window when minimized instead of staying on the taskbar"
        checked={current.minimizeToTray}
        onchange={(v) => set('minimizeToTray', v)}
      />
    </div>
  </section>

  <section class="card">
    <header class="card-header">
      <h2>Notifications</h2>
      <p>How alerts and unread indicators are displayed.</p>
    </header>
    <div class="card-body">
      <Toggle
        label="Show notification on new message"
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
    </div>
  </section>

  <section class="card">
    <header class="card-header">
      <h2>Appearance</h2>
      <p>Theme, zoom and window decorations.</p>
    </header>
    <div class="card-body">
      <label class="field">
        <span class="meta"><span class="label">Theme</span></span>
        <select
          class="control"
          value={current.theme}
          onchange={(e) => set('theme', (e.currentTarget as HTMLSelectElement).value as Theme)}
        >
          <option value="system">System</option>
          <option value="light">Light</option>
          <option value="dark">Dark</option>
        </select>
      </label>
      <label class="field">
        <span class="meta">
          <span class="label">Zoom level</span>
          <span class="description">Applied inside the Google Chat webview.</span>
        </span>
        <input
          class="control control-narrow"
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
        label="Always on top"
        description="Keep the window above other apps"
        checked={current.alwaysOnTop}
        onchange={(v) => set('alwaysOnTop', v)}
      />
      <Toggle
        label="Hide menu bar"
        description="Press Alt to reveal the menu temporarily"
        checked={current.hideMenuBar}
        onchange={(v) => set('hideMenuBar', v)}
      />
    </div>
  </section>

  <section class="card">
    <header class="card-header">
      <h2>Focus Mode</h2>
      <p>Temporarily mute notifications.</p>
    </header>
    <div class="card-body">
      <Toggle
        label="Active"
        description="Suppresses notifications for the configured duration"
        checked={current.focusMode}
        onchange={toggleFocusMode}
      />
      <label class="field">
        <span class="meta"><span class="label">Duration</span></span>
        <select
          class="control"
          value={String(current.focusModeDuration)}
          onchange={(e) =>
            set(
              'focusModeDuration',
              Number.parseInt((e.currentTarget as HTMLSelectElement).value, 10),
            )}
        >
          <option value="5">5 minutes</option>
          <option value="15">15 minutes</option>
          <option value="25">25 minutes</option>
          <option value="60">1 hour</option>
          <option value="120">2 hours</option>
        </select>
      </label>
    </div>
  </section>

  <section class="card">
    <header class="card-header">
      <h2>Advanced</h2>
      <p>Shortcut and CSS overrides for power users.</p>
    </header>
    <div class="card-body">
      <label class="field">
        <span class="meta">
          <span class="label">Global shortcut</span>
          <span class="description">Toggles the main window from anywhere in the system.</span>
        </span>
        <input
          class="control"
          type="text"
          value={current.globalShortcut}
          onchange={(e) => set('globalShortcut', (e.currentTarget as HTMLInputElement).value)}
        />
      </label>
      <label class="field column">
        <span class="meta">
          <span class="label">Custom CSS</span>
          <span class="description">Applied inside Google Chat after each navigation.</span>
        </span>
        <textarea
          class="control"
          rows="6"
          value={current.customCss}
          onchange={(e) => set('customCss', (e.currentTarget as HTMLTextAreaElement).value)}
        ></textarea>
      </label>
    </div>
  </section>
</main>

<style>
  main {
    padding: 1.5rem 2rem 3rem;
    max-width: 680px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  header {
    margin-bottom: 0.5rem;
  }
  h1 {
    margin: 0;
    font-size: 1.6rem;
  }
  .subtitle {
    margin: 0.25rem 0 0;
    color: var(--muted);
    font-size: 0.9rem;
  }
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
  }
  .card-header {
    padding: 1rem 1.25rem 0.5rem;
    margin: 0;
  }
  .card-header h2 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }
  .card-header p {
    margin: 0.15rem 0 0;
    font-size: 0.8rem;
    color: var(--muted);
  }
  .card-body {
    padding: 0.5rem 1.25rem 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
  }
  .field {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.6rem 0;
  }
  .field.column {
    flex-direction: column;
    align-items: stretch;
    gap: 0.5rem;
  }
  .meta {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }
  .meta .label {
    font-weight: 500;
    line-height: 1.2;
  }
  .meta .description {
    font-size: 0.8rem;
    opacity: 0.7;
    margin-top: 0.2rem;
    line-height: 1.3;
  }
  .meta code {
    font-size: 0.78rem;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 0 0.3rem;
  }
  .control {
    min-width: 180px;
  }
  .control-narrow {
    min-width: 96px;
    max-width: 96px;
    text-align: right;
  }
  textarea.control {
    width: 100%;
    min-width: 0;
    font-family: 'Cascadia Code', Consolas, monospace;
    font-size: 0.85rem;
    resize: vertical;
  }
  .control:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
  }
</style>
