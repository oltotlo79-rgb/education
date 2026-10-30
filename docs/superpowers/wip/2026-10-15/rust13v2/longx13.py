"""ファイルの中の x 10,001 個の並びを、先頭の 10 個と説明に置き換える（本文が長すぎて載せられないため）。"""
import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()
s = s.replace("x" * 10001, "x" * 10 + "…（x が全部で 10,001 個。ここでは先頭の 10 個だけ表示）")
open(p, "w", encoding="utf-8", newline="").write(s)
