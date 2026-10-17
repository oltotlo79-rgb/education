return (
  <ToastContext.Provider
    value={{ toasts, addToast, removeToast, success, error, warning, info }}
  >
    {children}
  </ToastContext.Provider>
);
