const variantStyles = {
  primary: `bg-primary-600 text-white hover:bg-primary-700 ...`,
  danger: `bg-red-600 text-white hover:bg-red-700 ...`,
  // ...
};
const sizeStyles = {
  sm: "px-3 py-1.5 text-xs gap-1.5",
  md: "px-4 py-2.5 text-sm gap-2",
  lg: "px-6 py-3 text-base gap-2.5",
};
// className 内で:
//   ${variantStyles[variant]}
//   ${sizeStyles[size]}
