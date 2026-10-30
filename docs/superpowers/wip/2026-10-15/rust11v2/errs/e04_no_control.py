import runpy, os
REPLACEMENTS = runpy.run_path(os.path.join(os.path.dirname(__file__), "e04_tables.py"))["NO_CONTROL"]
