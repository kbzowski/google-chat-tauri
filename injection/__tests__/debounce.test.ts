import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { debounce } from '../lib/debounce';

describe('debounce', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });
  afterEach(() => {
    vi.useRealTimers();
  });

  it('fires only once after the delay', () => {
    const spy = vi.fn();
    const d = debounce(spy, 100);
    d();
    d();
    d();
    expect(spy).not.toHaveBeenCalled();
    vi.advanceTimersByTime(100);
    expect(spy).toHaveBeenCalledTimes(1);
  });

  it('uses arguments from the last call', () => {
    const spy = vi.fn();
    const d = debounce(spy, 50);
    d(1 as never);
    d(2 as never);
    d(3 as never);
    vi.advanceTimersByTime(50);
    expect(spy).toHaveBeenCalledWith(3);
  });
});
