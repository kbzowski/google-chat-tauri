type TauriInvoke = (cmd: string) => Promise<unknown>;

interface TauriInternals {
  invoke?: TauriInvoke;
}

declare global {
  interface Window {
    __TAURI_INTERNALS__?: TauriInternals;
  }
}

export function installMenuToggle(): void {
  let altPressed = false;
  let altClean = true;

  const reset = () => {
    altPressed = false;
    altClean = true;
  };

  window.addEventListener(
    'keydown',
    (event) => {
      if (event.key === 'Alt') {
        if (!altPressed) {
          altPressed = true;
          altClean = true;
        }
      } else if (altPressed) {
        altClean = false;
      }
    },
    true,
  );

  window.addEventListener(
    'keyup',
    (event) => {
      if (event.key !== 'Alt') {
        if (altPressed) altClean = false;
        return;
      }
      const shouldToggle = altPressed && altClean;
      reset();
      if (!shouldToggle) return;
      const invoke = window.__TAURI_INTERNALS__?.invoke;
      if (invoke) {
        void invoke('toggle_main_menu');
      }
    },
    true,
  );

  window.addEventListener('blur', reset, true);
}
