import runpy, os
REPLACEMENTS = runpy.run_path(os.path.join(os.path.dirname(__file__), "e05_tables.py"))["NAV_LABELS"]
