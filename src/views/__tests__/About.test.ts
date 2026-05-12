import { render, waitFor } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';

vi.mock('../../lib/ipc', () => ({
  getDebugInfo: vi.fn(() =>
    Promise.resolve({
      appVersion: '0.0.1',
      firefoxVersion: '137.0',
      userAgent: 'Mozilla/5.0 Firefox/137.0',
      buildDate: '2026-05-12T00:00:00Z',
      platform: 'windows',
      webviewVersion: '125.0',
    }),
  ),
}));

vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({
  writeText: vi.fn(() => Promise.resolve()),
}));

vi.mock('@tauri-apps/plugin-opener', () => ({
  openUrl: vi.fn(() => Promise.resolve()),
}));

import About from '../About.svelte';

describe('About view', () => {
  it('shows debug info after async load', async () => {
    const { getByText } = render(About);
    await waitFor(() => {
      expect(getByText(/App Version: 0.0.1/)).toBeInTheDocument();
      expect(getByText(/Firefox UA: 137.0/)).toBeInTheDocument();
      expect(getByText(/Platform: windows/)).toBeInTheDocument();
    });
  });

  it('writeText is called when copy button clicked', async () => {
    const { writeText } = await import('@tauri-apps/plugin-clipboard-manager');
    const { findByRole } = render(About);
    const btn = await findByRole('button', { name: /copy to clipboard/i });
    btn.click();
    await waitFor(() => expect(writeText).toHaveBeenCalled());
  });
});
