"""ラベルと「状態 秒」の組の並びから、状態と、時間の中央値・最小・最大（ミリ秒）を表示する。"""
import statistics, sys

label, rest = sys.argv[1], sys.argv[2:]
codes = rest[0::2]
ms = [float(t) * 1000 for t in rest[1::2]]
print(f"{label}: 状態 {'・'.join(sorted(set(codes)))}・中央値 {statistics.median(ms):.0f} ミリ秒（{len(ms)} 回: {min(ms):.0f}〜{max(ms):.0f}）")
