import runpy, os
REPLACEMENTS = runpy.run_path(os.path.join(os.path.dirname(__file__), "e13b_tables.py"))["PINNED_0002"]
