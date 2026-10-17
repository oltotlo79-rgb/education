const addToast = useCallback(
  (type, title, message, duration = 5000) => {
    const id = generateToastId();
    const newToast: Toast = { id, type, title, message, duration };
    setToasts((prev) => [...prev, newToast]);
    if (duration > 0) {
      setTimeout(() => { removeToast(id); }, duration);
    }
  },
  [removeToast]
);
