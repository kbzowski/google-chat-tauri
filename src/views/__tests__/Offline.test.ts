import { fireEvent, render, waitFor } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';

vi.mock('../../lib/ipc', () => ({
  checkIfOnline: vi.fn(() => Promise.resolve(true)),
  emit: vi.fn(() => Promise.resolve()),
}));

vi.mock('@tauri-apps/plugin-opener', () => ({
  openUrl: vi.fn(() => Promise.resolve()),
}));

import Offline from '../Offline.svelte';

describe('Offline view', () => {
  it('renders Try Now button', () => {
    const { getByRole } = render(Offline);
    expect(getByRole('button', { name: /try now/i })).toBeInTheDocument();
  });

  it('clicking Try Now calls checkIfOnline', async () => {
    const { checkIfOnline } = await import('../../lib/ipc');
    const { getByRole } = render(Offline);
    await fireEvent.click(getByRole('button', { name: /try now/i }));
    await waitFor(() => expect(checkIfOnline).toHaveBeenCalled());
  });

  it('emits connection-restored when ping succeeds', async () => {
    const { emit } = await import('../../lib/ipc');
    const { getByRole } = render(Offline);
    await fireEvent.click(getByRole('button', { name: /try now/i }));
    await waitFor(() => expect(emit).toHaveBeenCalledWith('connection-restored'));
  });

  it('Open in Browser button calls openUrl', async () => {
    const { openUrl } = await import('@tauri-apps/plugin-opener');
    const { getByRole } = render(Offline);
    await fireEvent.click(getByRole('button', { name: /open in browser/i }));
    expect(openUrl).toHaveBeenCalledWith('https://chat.google.com');
  });
});
