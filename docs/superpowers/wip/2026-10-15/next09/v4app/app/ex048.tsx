const [isExiting, setIsExiting] = useState(false);
const handleClose = useCallback(() => {
  setIsExiting(true);
  setTimeout(() => {
    removeToast(toast.id);
  }, 300);
}, [removeToast, toast.id]);
