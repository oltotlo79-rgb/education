import sys
data = sys.stdin.buffer.read().decode("utf-8", errors="replace")
root = r"C:\Users\oltot\AppData\Local\Temp\claude\C--Users-oltot-Documents-git-projects-def\f344debc-e3c6-4c0d-93d7-e55dbf9acedd\scratchpad\rust00"
pairs = [
    (root + r"\study", r"C:\Users\you\rust-study"),
    (root + r"\home", r"C:\Users\you"),
]
for a, b in pairs:
    data = data.replace(a, b)
    data = data.replace(a.replace("\\", "/"), b.replace("\\", "/"))
data = data.replace(r"C:\Users\oltot", r"C:\Users\you")
data = data.replace("C:/Users/oltot", "C:/Users/you")
data = data.replace("/c/Users/oltot", "/c/Users/you")
sys.stdout.write(data)
