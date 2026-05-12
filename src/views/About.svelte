<script lang="ts">
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { openUrl } from '@tauri-apps/plugin-opener';
import { onMount } from 'svelte';
import { type DebugInfo, getDebugInfo } from '../lib/ipc';

const REPO_URL = 'https://github.com/kbzowski/google-chat-tauri';

let info = $state<DebugInfo | null>(null);
let copied = $state(false);

onMount(async () => {
  try {
    info = await getDebugInfo();
  } catch {
    info = null;
  }
});

function formatDebugBlock(d: DebugInfo): string {
  return [
    `App Version: ${d.appVersion}`,
    `Firefox UA: ${d.firefoxVersion}`,
    `Build Date: ${d.buildDate}`,
    `Platform: ${d.platform}`,
    `WebView: ${d.webviewVersion}`,
    `User-Agent: ${d.userAgent}`,
  ].join('\n');
}

async function copyDebug() {
  if (!info) return;
  await writeText(formatDebugBlock(info));
  copied = true;
  setTimeout(() => {
    copied = false;
  }, 1500);
}

function openRepo() {
  openUrl(REPO_URL).catch(() => {});
}
</script>

<main>
  <header>
    <h1>google-chat-tauri</h1>
    <p class="version">{info ? `version ${info.appVersion}` : 'loading…'}</p>
    <p class="license">MIT License</p>
  </header>

  <section>
    <h2>Debug info</h2>
    {#if info}
      <pre>{formatDebugBlock(info)}</pre>
      <button onclick={copyDebug}>
        {copied ? 'Copied!' : 'Copy to Clipboard'}
      </button>
    {:else}
      <p>Loading debug info…</p>
    {/if}
  </section>

  <footer>
    <button onclick={openRepo}>Open GitHub</button>
  </footer>
</main>

<style>
  main {
    padding: 1.5rem 2rem;
    max-width: 480px;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  header {
    text-align: center;
  }
  h1 {
    margin: 0;
    font-size: 1.4rem;
  }
  .version {
    margin: 0.25rem 0;
    color: var(--muted);
  }
  .license {
    margin: 0;
    color: var(--muted);
    font-size: 0.85rem;
  }
  h2 {
    margin: 0 0 0.5rem;
    font-size: 0.85rem;
    text-transform: uppercase;
    color: var(--muted);
    letter-spacing: 0.04em;
  }
  pre {
    background: var(--surface);
    padding: 0.75rem;
    border-radius: 4px;
    font-size: 0.85rem;
    white-space: pre-wrap;
    word-break: break-all;
    border: 1px solid var(--border);
  }
  footer {
    display: flex;
    justify-content: center;
  }
</style>
