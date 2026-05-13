import { render } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';

import Shortcuts from '../Shortcuts.svelte';

describe('Shortcuts view', () => {
  it('renders Window/Navigation/Zoom/Search sections', () => {
    const { getByText } = render(Shortcuts);
    expect(getByText('Window')).toBeInTheDocument();
    expect(getByText('Navigation')).toBeInTheDocument();
    expect(getByText('Zoom')).toBeInTheDocument();
    expect(getByText('Search')).toBeInTheDocument();
  });

  it('renders well-known shortcut keys', () => {
    const { getByText } = render(Shortcuts);
    expect(getByText('Ctrl+W')).toBeInTheDocument();
    expect(getByText('Ctrl+F')).toBeInTheDocument();
    expect(getByText('Alt+Left')).toBeInTheDocument();
  });
});
