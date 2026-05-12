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
  <input
    type="checkbox"
    {checked}
    onchange={(e) => onchange((e.currentTarget as HTMLInputElement).checked)}
  />
  <span class="meta">
    <span class="label">{label}</span>
    {#if description}
      <span class="description">{description}</span>
    {/if}
  </span>
</label>

<style>
  .toggle {
    display: flex;
    gap: 0.75rem;
    padding: 0.5rem 0;
    align-items: flex-start;
    cursor: pointer;
  }
  .meta {
    display: flex;
    flex-direction: column;
  }
  .label {
    font-weight: 500;
  }
  .description {
    font-size: 0.85rem;
    opacity: 0.7;
  }
</style>
