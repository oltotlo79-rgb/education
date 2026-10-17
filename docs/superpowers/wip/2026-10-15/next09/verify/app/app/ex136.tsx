const dialogRef = useRef<HTMLDivElement>(null);
const cancelButtonRef = useRef<HTMLButtonElement>(null);

useEffect(() => {
  if (isOpen) {
    cancelButtonRef.current?.focus();
  }
}, [isOpen]);
