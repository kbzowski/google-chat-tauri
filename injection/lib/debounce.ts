export function debounce<T extends (...args: never[]) => void>(fn: T, ms: number): T {
  let handle: ReturnType<typeof setTimeout> | null = null;
  return ((...args: Parameters<T>) => {
    if (handle !== null) clearTimeout(handle);
    handle = setTimeout(() => fn(...args), ms);
  }) as T;
}
