import sys

p = sys.argv[1]
t = open(p, encoding="utf-8").read()


def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)


rep(
    """    #[test]
    fn note_title_counts_characters_not_bytes() {
        // 日本語 80 文字はちょうど上限（バイト数では 240）
        assert!(NoteTitle::parse(&"あ".repeat(80)).is_ok());
        // 81 文字は長すぎる
        assert_eq!(
            NoteTitle::parse(&"あ".repeat(81)),
            Err(TitleError::TooLong { len: 81, max: 80 })
        );
    }""",
    """    #[test]
    fn note_title_counts_characters_not_bytes() {
        // 日本語 80 文字は 240 バイトだが、文字数では 80 なので合格
        let eighty = "あ".repeat(80);
        assert!(NoteTitle::parse(&eighty).is_ok());

        let eighty_one = "あ".repeat(81);
        assert_eq!(
            NoteTitle::parse(&eighty_one),
            Err(TitleError::TooLong { len: 81, max: 80 })
        );
    }""",
)
rep(
    "- **UTF-16 の数との食い違い:** ブラウザの入力欄の `maxlength`（入力できる長さの上限）は、HTML の決まりでは UTF-16 の単位で数えます。RustNote の作成のフォームは `maxlength=(MAX_TITLE_CHARS)`（80）を付けているので、👍 のように UTF-16 で 2 単位の絵文字だけのタイトルは、ブラウザでは 40 個までしか入りません。サーバーは 80 文字（80 個）まで受け付けるので、ブラウザのほうが先に止めることになります。どの数え方で「長さ」を決めるかは、仕様として決めて、表のテストに書いておきます。",
    "- **UTF-16 の数との食い違い:** ブラウザの入力欄の `maxlength`（入力できる長さの上限）は、HTML の決まりでは UTF-16 の単位で数えます。RustNote の作成のフォームは `maxlength=(MAX_TITLE_CHARS)`（80）を付けているので、決まりどおりに数えるブラウザでは、👍 のように UTF-16 で 2 単位の絵文字だけのタイトルは、40 個までしか入りません。サーバーは 80 文字（80 個）まで受け付けるので、ブラウザのほうが先に止めることになります。実際のブラウザでの数え方は、ブラウザを使ったテストで確かめます（UNIT 05（準備中））。どの数え方で「長さ」を決めるかは、仕様として決めて、表のテストに書いておきます。",
)
open(p, "w", encoding="utf-8", newline="").write(t)
print("ok")
