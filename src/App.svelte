<script lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import About from './views/About.svelte';
import Offline from './views/Offline.svelte';
import Settings from './views/Settings.svelte';
import Shortcuts from './views/Shortcuts.svelte';

let label = $state('');
try {
  label = getCurrentWebviewWindow().label;
} catch {
  label = '';
}
</script>

{#if label === 'settings'}
  <Settings />
{:else if label === 'about'}
  <About />
{:else if label === 'offline'}
  <Offline />
{:else if label === 'shortcuts'}
  <Shortcuts />
{:else}
  <main class="placeholder">
    <h1>google-chat-tauri</h1>
    <p>Unknown window label: <code>{label || '(none)'}</code></p>
  </main>
{/if}

<style>
  .placeholder {
    font-family: system-ui, sans-serif;
    padding: 1rem;
  }
</style>
