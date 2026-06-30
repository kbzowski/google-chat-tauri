import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { debounce } from './lib/debounce';
import { logger } from './lib/logger';

const log = logger('injection::spellcheck');
export const EVENT_APPLY = 'apply-spellcheck';
const EDITABLE = '[contenteditable=""], [contenteditable="true"], textarea, input';

let disabled = false;

function applyTo(root: ParentNode): void {
  for (const el of root.querySelectorAll<HTMLElement>(EDITABLE)) {
    el.spellcheck = !disabled;
  }
}

export function installSpellcheck(): void {
  invoke<boolean>('is_spell_check_disabled')
    .then((value) => {
      disabled = value;
      applyTo(document);
    })
    .catch(() => {});

  listen<boolean>(EVENT_APPLY, ({ payload }) => {
    disabled = payload;
    log.debug('Spell check toggled', { disabled });
    applyTo(document);
  }).catch(() => {});

  // Google Chat re-renders its composer; re-assert the override on new nodes
  // while disabled (enabled is the browser default, so no upkeep is needed).
  const reapply = debounce(() => {
    if (disabled) applyTo(document);
  }, 250);
  new MutationObserver(reapply).observe(document.body, { childList: true, subtree: true });
}
