<script lang="ts">
interface Props {
  label: string;
  description?: string;
  checked: boolean;
  onchange: (value: boolean) => void;
}
const { label, description, checked, onchange }: Props = $props();
</script>

<label class="toggle">
  <span class="meta">
    <span class="label">{label}</span>
    {#if description}
      <span class="description">{description}</span>
    {/if}
  </span>
  <span class="switch" class:on={checked}>
    <input
      type="checkbox"
      {checked}
      onchange={(e) => onchange((e.currentTarget as HTMLInputElement).checked)}
    />
    <span class="track"></span>
    <span class="thumb"></span>
  </span>
</label>

<style>
  .toggle {
    display: flex;
    gap: 1rem;
    padding: 0.6rem 0;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    user-select: none;
  }
  .toggle:hover .track {
    border-color: var(--accent);
  }
  .meta {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }
  .label {
    font-weight: 500;
    line-height: 1.2;
  }
  .description {
    font-size: 0.8rem;
    opacity: 0.7;
    margin-top: 0.2rem;
    line-height: 1.3;
  }
  .switch {
    position: relative;
    width: 38px;
    height: 22px;
    flex-shrink: 0;
  }
  .switch input {
    position: absolute;
    inset: 0;
    opacity: 0;
    margin: 0;
    cursor: pointer;
  }
  .track {
    position: absolute;
    inset: 0;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 11px;
    transition:
      background 0.15s ease,
      border-color 0.15s ease;
  }
  .switch.on .track {
    background: var(--accent);
    border-color: var(--accent);
  }
  .thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: var(--bg);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: transform 0.15s ease;
    pointer-events: none;
  }
  .switch.on .thumb {
    transform: translateX(16px);
    background: #ffffff;
  }
  .switch input:focus-visible + .track {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
