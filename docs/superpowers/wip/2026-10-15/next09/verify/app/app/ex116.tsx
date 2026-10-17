export function AnimatedButton({
  variant = "primary",
  size = "md",
  isLoading = false,
  children,
  className = "",
  disabled,
  ...props
}: AnimatedButtonProps) {
  return (
    <button
      // ... className ...
      disabled={disabled || isLoading}
      {...props}
    >
