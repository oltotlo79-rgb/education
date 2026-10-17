const [theme, setTheme] = useState<Theme>("system");
const [mounted, setMounted] = useState(false);

useEffect(() => {
  setMounted(true);
  const savedTheme = localStorage.getItem("theme") as Theme | null;
  if (savedTheme) {
    setTheme(savedTheme);
    applyTheme(savedTheme);
  } else {
    applyTheme("system");
  }
}, []);
