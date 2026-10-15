function findById<T extends { id: string }>(items: T[], id: string): T | undefined {
  return items.find((item) => item.id === id);
}

// 数値の配列には id が無いので、T の条件を満たさない
findById([1, 2, 3], "1");
