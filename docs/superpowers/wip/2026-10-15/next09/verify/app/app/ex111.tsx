useEffect(() => {
  setIsVisible(false);
  setDisplayChildren(children);
  const timer = requestAnimationFrame(() => {
    setIsVisible(true);
  });
  return () => cancelAnimationFrame(timer);
}, [pathname, children]);
