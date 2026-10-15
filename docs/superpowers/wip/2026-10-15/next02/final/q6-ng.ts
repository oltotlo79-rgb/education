function last<T>(items: T[]): T | undefined {
  if (items.length === 0) {
    return undefined;
  }
  return items[items.length - 1];
}

last<string>([1, 2]);
