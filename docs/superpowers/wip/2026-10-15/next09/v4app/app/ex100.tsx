const linear = (c: number) =>
  c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
return 0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b);
