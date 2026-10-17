const light = Math.max(a, b);
const dark = Math.min(a, b);
return (light + 0.05) / (dark + 0.05);
