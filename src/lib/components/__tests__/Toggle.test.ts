import { fireEvent, render } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import Toggle from '../Toggle.svelte';

describe('Toggle', () => {
  it('renders the label and description', () => {
    const { getByLabelText, getByText } = render(Toggle, {
      props: {
        label: 'Always on Top',
        description: 'Keeps the window above other apps',
        checked: false,
        onchange: vi.fn(),
      },
    });
    expect(getByLabelText(/always on top/i)).toBeInTheDocument();
    expect(getByText(/keeps the window/i)).toBeInTheDocument();
  });

  it('calls onchange with new state when toggled', async () => {
    const onchange = vi.fn();
    const { getByRole } = render(Toggle, {
      props: { label: 'X', checked: false, onchange },
    });
    const checkbox = getByRole('checkbox') as HTMLInputElement;
    await fireEvent.click(checkbox);
    expect(onchange).toHaveBeenCalledWith(true);
  });
});
