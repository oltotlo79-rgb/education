import io
import os

HERE = os.path.dirname(os.path.abspath(__file__))
TARGET = r"C:/Users/oltot/Documents/git-projects/def/next/08-crud-edit-delete.md"


def blk(name):
    return io.open(os.path.join(HERE, name), encoding="utf-8").read().strip("\n")


text = io.open(TARGET, encoding="utf-8").read()
lines = text.split("\n")


def find_line(prefix, start=0):
    hits = [i for i, l in enumerate(lines) if l.startswith(prefix) and i >= start]
    assert len(hits) == 1, (prefix, hits)
    return hits[0]


def insert_after(prefix, content):
    i = find_line(prefix)
    new = content.split("\n")
    lines[i + 1:i + 1] = [""] + new
    print("inserted after line", i + 1, ":", prefix[:30])


def insert_before(idx, content):
    new = content.split("\n")
    lines[idx:idx] = new + [""]


# 後ろから順に挿入すると行番号がずれないが、ここでは前方一致で毎回探すので順不同で OK
insert_after("> Next.js では、これらを意識せずに JavaScript の関数として呼び出せる場面が多いですが", blk("b01_goal.md"))
insert_after("8. [全体のページ遷移図](#8-全体のページ遷移図)", "9. [練習問題（解答・解説つき）](#9-練習問題解答解説つき)")
# 目次の項目は空行なしでつなげる
i = find_line("9. [練習問題（解答・解説つき）]")
assert lines[i - 1] == ""
del lines[i - 1]

insert_after("> **なぜ Promise なの？**", blk("b03_routing.md"))
insert_after("ページ上部にはグラデーション背景のヘッダーがあり", blk("b04_notfound.md"))
insert_after("このフロー図が示すように、編集機能は2段階の処理で構成されています。", blk("b05_editflow.md"))
insert_after("> **楽観的 UI（Optimistic UI）について:**", blk("b06_optimistic.md"))
insert_after("> **用語:** **三項演算子（`?:`）**", "---\n\n" + blk("b07_formstate.md"))
insert_after("- 戻るリンクは詳細ページ（`/books/${id}`）を指しています。", blk("b08_editscreen.md"))
insert_after("> **用語:** **役割分担（Server/Client の分離）**", "---\n\n" + blk("b09_convert.md") + "\n\n---\n\n" + blk("b10_serveraction.md"))
insert_after("削除ボタンを押すと、まず `window.confirm()` による確認ダイアログが表示されます。", blk("b11_dialog.md"))
insert_after("> **用語:** **`router.push()`** はプログラムからの画面遷移", "---\n\n" + blk("b12_delete.md"))

# 7-1: Next.js 16 の注意
insert_after(
    "- **Next.js 14 以前**: `params` は通常のオブジェクトとして渡されるため",
    "> **注意:** Next.js 15 では、移行を助けるために「`await` せずに `params.id` と読む古い書き方」も一時的に動いていましたが、**Next.js 16 でこの互換措置は完全に削除されました**。現在の Next.js では必ず `await params`（クライアントコンポーネントのページなら React の `use(params)`）で取り出してください。",
)

# 7-2: コメントの誤りを修正
i = find_line("// data が [] なら該当 ID 無し、null なら RLS で弾かれている可能性")
lines[i:i + 1] = [
    "// error があれば失敗。error が無くても data が [] なら、",
    "// 「該当する id が無い」か「RLS で対象の行が見えず 0 件だった」可能性",
]

# 7-6: 制約違反（## 8 の直前の --- の前）
i = find_line("## 8. 全体のページ遷移図")
j = i - 1
while lines[j] != "---":
    j -= 1
insert_before(j, blk("b15_constraint.md") + "\n")

# 9: 練習問題（8 の最後の段落の後）
insert_after("3. **一覧 → 詳細 → 削除 → 一覧**:", "---\n\n" + blk("b16_practice.md"))

# まとめの表に not-found.tsx を追加
insert_after("| 削除 | `components/DeleteButton.tsx` |", "| 404 画面 | `app/books/[id]/not-found.tsx` | 存在しない id を開いたときの案内画面 |")
i = find_line("| 404 画面 |")
assert lines[i - 1] == ""
del lines[i - 1]

io.open(TARGET, "w", encoding="utf-8", newline="\n").write("\n".join(lines))
print("total lines", len(lines))
