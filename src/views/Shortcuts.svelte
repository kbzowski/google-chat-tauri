<script lang="ts">
interface Item {
  keys: string;
  description: string;
}
interface Section {
  title: string;
  items: Item[];
}

const sections: Section[] = [
  {
    title: 'Window',
    items: [
      { keys: 'Ctrl+W', description: 'Close to tray' },
      { keys: 'Ctrl+Q', description: 'Quit' },
      { keys: 'F11', description: 'Toggle fullscreen' },
    ],
  },
  {
    title: 'Navigation',
    items: [
      { keys: 'Alt+Left', description: 'Back' },
      { keys: 'Alt+Right', description: 'Forward' },
      { keys: 'Alt+Home', description: 'Navigate to home' },
      { keys: 'Ctrl+R', description: 'Reload page' },
    ],
  },
  {
    title: 'Zoom',
    items: [
      { keys: 'Ctrl++', description: 'Zoom in' },
      { keys: 'Ctrl+-', description: 'Zoom out' },
      { keys: 'Ctrl+0', description: 'Reset zoom' },
    ],
  },
  {
    title: 'Search',
    items: [{ keys: 'Ctrl+F', description: 'Focus search input' }],
  },
];
</script>

<main>
  <h1>Keyboard Shortcuts</h1>
  {#each sections as section}
    <section>
      <h2>{section.title}</h2>
      <dl>
        {#each section.items as item}
          <dt><kbd>{item.keys}</kbd></dt>
          <dd>{item.description}</dd>
        {/each}
      </dl>
    </section>
  {/each}
  <p class="note">
    Global shortcut (default <kbd>Ctrl+Shift+G</kbd>) toggles the main window. Configure it in
    Settings → Advanced.
  </p>
</main>

<style>
  main {
    padding: 1.5rem 2rem;
    max-width: 480px;
  }
  h1 {
    margin: 0 0 1rem;
    font-size: 1.3rem;
  }
  h2 {
    margin: 1.25rem 0 0.5rem;
    font-size: 0.85rem;
    text-transform: uppercase;
    color: var(--muted);
    letter-spacing: 0.04em;
  }
  dl {
    margin: 0;
    display: grid;
    grid-template-columns: 8rem 1fr;
    row-gap: 0.4rem;
    column-gap: 1rem;
  }
  dt {
    margin: 0;
  }
  dd {
    margin: 0;
  }
  kbd {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 0.1rem 0.4rem;
    font-family: 'Cascadia Code', Consolas, monospace;
    font-size: 0.85rem;
  }
  .note {
    margin: 1.5rem 0 0;
    font-size: 0.85rem;
    color: var(--muted);
  }
</style>
