<script lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { openUrl } from '@tauri-apps/plugin-opener';
import { onDestroy, onMount } from 'svelte';
import { checkIfOnline, emit } from '../lib/ipc';

const CHAT_URL = 'https://chat.google.com';
const RETRY_AFTER_SECONDS = 30;

let checking = $state(false);
let countdown = $state(RETRY_AFTER_SECONDS);
let intervalId: ReturnType<typeof setInterval> | null = null;

async function retry(): Promise<void> {
  if (checking) return;
  checking = true;
  try {
    const online = await checkIfOnline();
    if (online) {
      emit('connection-restored').catch(() => {});
      try {
        await getCurrentWebviewWindow().close();
      } catch {
        // window may already be closing
      }
    }
  } finally {
    checking = false;
    countdown = RETRY_AFTER_SECONDS;
  }
}

function startCountdown(): void {
  intervalId = setInterval(() => {
    countdown -= 1;
    if (countdown <= 0) retry();
  }, 1000);
}

function openInBrowser(): void {
  openUrl(CHAT_URL).catch(() => {});
}

onMount(startCountdown);
onDestroy(() => {
  if (intervalId !== null) clearInterval(intervalId);
});
</script>

<main>
  <div class="icon">⚡</div>
  <h1>You're offline</h1>
  <p>Trying to reconnect in {countdown}s...</p>
  <div class="actions">
    <button onclick={retry} disabled={checking}>
      {checking ? 'Checking…' : 'Try Now'}
    </button>
    <button onclick={openInBrowser}>Open in Browser</button>
  </div>
</main>

<style>
  main {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
  }
  .icon {
    font-size: 3rem;
    opacity: 0.5;
  }
  h1 {
    margin: 0;
    font-size: 1.4rem;
  }
  p {
    margin: 0;
    color: var(--muted);
  }
  .actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
  }
</style>
