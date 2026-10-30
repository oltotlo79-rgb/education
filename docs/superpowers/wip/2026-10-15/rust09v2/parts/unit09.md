## UNIT 09: デザインの決まり・アセット・アクセシビリティ

### この単元のゴール

- RustNote の見た目の決まり（色の変数・ボタンの3種類・エラーとお知らせの出し方）を、`assets/style.css` から読み取り、新しい画面でも同じ決まりを使える
- 文字と背景の**コントラスト比**を計算し、WCAG（Web の読みやすさの国際的な決まり）の AA（4.5 以上）を満たすかを、数で確かめられる
- どのページも「`lang` がある・`<h1>` が1つ・入力欄にラベルがある・ボタンに文字がある」かを、本物の HTML から確かめるテストを書ける
- 確かめで見つかった RustNote の改善点（検索の入力欄のラベル・ページごとに同じ `<title>`）を説明できる

### たとえ話でつかむ

駅の案内表示を考えます。

- どの駅でも、**色の決まり**があります。乗り換えは黄色、出口は緑、非常口は赤。駅ごとに色を変えると、毎回読み方を覚え直さなければなりません。だから、色は少なく、意味は1つに決めます。
- 案内の文字は、**遠くからでも読める濃さ**にします。薄い灰色の上に、少しだけ濃い灰色で書くと、目の弱い人や、まぶしい日には読めません。濃さの差は、数で決めます。
- 目の見えない人のために、**音声の案内**や点字があります。「ここは2番線」と、場所の名前が付いています。名前の無い表示は、音声では読めません。
- 案内は、**作ったら、決まりの表で点検**します。「全部の出口に名前があるか」を、目で全部見て回るのは大変なので、表に沿って、順に確かめます。

| 駅の案内 | RustNote |
|---|---|
| 色の決まり | `:root` の色の変数（`--color-primary` など）と、ボタンの3種類 |
| 遠くから読める濃さ | コントラスト比（文字と背景の明るさの差）。4.5 以上 |
| 場所の名前 | `<label for="id">`・`aria-label`・`<h1>`・`lang="ja"` |
| 決まりの表で点検 | `tests/u09_a11y.rs`（全部のページの HTML を確かめる） |

> **用語:** アクセシビリティ（accessibility：目・耳・手の使い方が人によって違っても、誰でも使えるようにすること。画面の読み上げソフトや、キーボードだけでの操作でも使えるようにする）

<!-- -->

> **用語:** コントラスト比（文字と背景の明るさの比。1（同じ色）から 21（白と黒）まで。WCAG の AA は、ふつうの大きさの文字で 4.5 以上を求める）

<!-- -->

> **用語:** WCAG（ダブリューキャグ：Web Content Accessibility Guidelines。Web のアクセシビリティの国際的な決まり。A・AA・AAA の3つの段階がある）

### 図で理解する

RustNote の色の決まりです（`assets/style.css` の `:root`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="680" height="210" rx="10" fill="#f8fafc" stroke="#e2e8f0" stroke-width="2"/>
<rect x="40" y="40" width="40" height="40" rx="6" fill="#0f172a"/>
<text x="92" y="56" font-size="12" font-weight="700" fill="#0f172a">--color-text</text>
<text x="92" y="74" font-size="11" fill="#475569">本文の文字</text>
<rect x="260" y="40" width="40" height="40" rx="6" fill="#64748b"/>
<text x="312" y="56" font-size="12" font-weight="700" fill="#0f172a">--color-muted</text>
<text x="312" y="74" font-size="11" fill="#475569">日時・補足の薄い文字</text>
<rect x="480" y="40" width="40" height="40" rx="6" fill="#c2410c"/>
<text x="532" y="56" font-size="12" font-weight="700" fill="#0f172a">--color-primary</text>
<text x="532" y="74" font-size="11" fill="#475569">リンク・主なボタン</text>
<rect x="40" y="100" width="40" height="40" rx="6" fill="#dc2626"/>
<text x="92" y="116" font-size="12" font-weight="700" fill="#0f172a">--color-danger</text>
<text x="92" y="134" font-size="11" fill="#475569">削除・入力のエラー</text>
<rect x="260" y="100" width="40" height="40" rx="6" fill="#ecfdf5" stroke="#047857" stroke-width="2"/>
<text x="312" y="116" font-size="12" font-weight="700" fill="#0f172a">--color-success-*</text>
<text x="312" y="134" font-size="11" fill="#475569">成功のお知らせ</text>
<rect x="480" y="100" width="40" height="40" rx="6" fill="#fffbeb" stroke="#b45309" stroke-width="2"/>
<text x="532" y="116" font-size="12" font-weight="700" fill="#0f172a">--color-warning-*</text>
<text x="532" y="134" font-size="11" fill="#475569">注意のお知らせ</text>
<rect x="40" y="170" width="120" height="36" rx="6" fill="#c2410c"/>
<text x="100" y="193" font-size="12" fill="#ffffff" text-anchor="middle">作成する</text>
<rect x="180" y="170" width="120" height="36" rx="6" fill="#ffffff" stroke="#c2410c" stroke-width="2"/>
<text x="240" y="193" font-size="12" fill="#c2410c" text-anchor="middle">完了にする</text>
<rect x="320" y="170" width="120" height="36" rx="6" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
<text x="380" y="193" font-size="12" fill="#dc2626" text-anchor="middle">削除</text>
<text x="460" y="186" font-size="11" fill="#475569">ボタンは3種類だけ:</text>
<text x="460" y="204" font-size="11" fill="#475569">主（button）・副（secondary）・危険（danger）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: RustNote の色とボタンの決まり。色は変数にまとめて、意味ごとに1つずつ。ボタンは、主・副・危険の3種類だけにして、どの画面でも同じ意味で使う</div>
</div>

アクセシビリティの最低限の決まりと、確かめ方です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="190" y="24" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">テストで自動で確かめる（HTML を読む）</text>
<rect x="20" y="36" width="340" height="180" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="40" y="66" font-size="12" fill="#0f172a">・&lt;html lang="ja"&gt;（読み上げの言葉を決める）</text>
<text x="40" y="94" font-size="12" fill="#0f172a">・&lt;h1&gt; がちょうど1つ（ページの題）</text>
<text x="40" y="122" font-size="12" fill="#0f172a">・入力欄に &lt;label for&gt; か aria-label</text>
<text x="40" y="150" font-size="12" fill="#0f172a">・ボタンに文字がある</text>
<text x="40" y="178" font-size="12" fill="#0f172a">・色のコントラスト比が 4.5 以上（計算）</text>
<text x="530" y="24" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">人が確かめる（ブラウザで）</text>
<rect x="380" y="36" width="320" height="180" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="400" y="66" font-size="12" fill="#0f172a">・Tab キーだけで、全部の操作ができる</text>
<text x="400" y="94" font-size="12" fill="#0f172a">・今どこにいるか（フォーカスの枠）が見える</text>
<text x="400" y="122" font-size="12" fill="#0f172a">・読み上げソフトで、意味が通じる</text>
<text x="400" y="150" font-size="12" fill="#0f172a">・狭い画面（スマートフォン）でも使える</text>
<text x="400" y="178" font-size="12" fill="#0f172a">→ 第10巻 UNIT 10（準備中）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 確かめ方の2つ。HTML と色で決まることはテストで毎回確かめ、操作や見え方は、ブラウザで人が確かめる</div>
</div>

### まず動かす最小コード

#### 例9-1: 色の組み合わせのコントラスト比を計算する

RustNote の色は、`assets/style.css` の最初に、変数としてまとめてあります（抜き出しています）。

```css
:root {
  --color-text: #0f172a;
  --color-muted: #64748b;
  --color-border: #e2e8f0;
  --color-bg: #f8fafc;
  --color-card: #ffffff;
  --color-primary: #c2410c;
  --color-primary-dark: #9a3412;
  --color-success-bg: #ecfdf5;
  --color-success-text: #047857;
  --color-warning-bg: #fffbeb;
  --color-warning-text: #b45309;
  --color-danger: #dc2626;
}
```

> **▼ このコードがやること（先に日本語で）:** `style.css` から `--color-…` の変数を読み出し、WCAG の計算式で、色ごとの相対輝度（明るさ）を出します。RustNote で実際に使っている、文字と背景の9つの組み合わせについて、コントラスト比を計算し、4.5 以上なら `ok` と表示します。

```python
"""RustNote の style.css の色の組み合わせの、コントラスト比（WCAG 2）を計算する。
使い方: python contrast.py rustnote/assets/style.css
"""
import re
import sys

css = open(sys.argv[1], encoding="utf-8").read()
colors = dict(re.findall(r"--(color-[a-z-]+):\s*(#[0-9a-fA-F]{6})", css))
colors["white"] = "#ffffff"


def luminance(hex_color):
    """相対輝度（0 が黒、1 が白）"""
    rgb = [int(hex_color[i : i + 2], 16) / 255 for i in (1, 3, 5)]
    lin = [c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4 for c in rgb]
    return 0.2126 * lin[0] + 0.7152 * lin[1] + 0.0722 * lin[2]


def ratio(fg, bg):
    a, b = luminance(colors[fg]), luminance(colors[bg])
    hi, lo = max(a, b), min(a, b)
    return (hi + 0.05) / (lo + 0.05)


# どこで使っている組み合わせか
pairs = [
    ("color-text", "color-bg", "本文の文字"),
    ("color-muted", "color-bg", "日時などの薄い文字"),
    ("color-muted", "color-card", "カードの中の薄い文字"),
    ("color-primary", "color-bg", "リンク"),
    ("white", "color-primary", "ボタンの文字"),
    ("color-primary", "color-card", "白いボタン（secondary）の文字"),
    ("color-danger", "color-card", "削除のボタン・エラーの文字"),
    ("color-success-text", "color-success-bg", "成功のお知らせ"),
    ("color-warning-text", "color-warning-bg", "注意のお知らせ"),
]
print("組み合わせ                              比     AA（4.5 以上）")
for fg, bg, where in pairs:
    r = ratio(fg, bg)
    mark = "ok" if r >= 4.5 else "NG"
    print(f"{where:<16} {colors[fg]} / {colors[bg]}  {r:5.2f}  {mark}")
```

コピーした `rustnote` の1つ上のフォルダで、Python で動かします（Python が無ければ、表示の結果だけを読んでもかまいません）。

```text
### python contrast.py rustnote/assets/style.css
組み合わせ                              比     AA（4.5 以上）
本文の文字            #0f172a / #f8fafc  17.06  ok
日時などの薄い文字        #64748b / #f8fafc   4.55  ok
カードの中の薄い文字       #64748b / #ffffff   4.76  ok
リンク              #c2410c / #f8fafc   4.95  ok
ボタンの文字           #ffffff / #c2410c   5.18  ok
白いボタン（secondary）の文字 #c2410c / #ffffff   5.18  ok
削除のボタン・エラーの文字    #dc2626 / #ffffff   4.83  ok
成功のお知らせ          #047857 / #ecfdf5   5.21  ok
注意のお知らせ          #b45309 / #fffbeb   4.84  ok
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 9つとも、AA（4.5 以上）を満たしている

- 本文の文字は 17.06 で、とても読みやすい濃さです。ボタン（白い文字と `--color-primary`）は 5.18、成功・注意のお知らせも 4.8〜5.2 です。
- いちばん低いのは、日時などの薄い文字（`--color-muted` と背景）の **4.55** です。AA の 4.5 を、ぎりぎり満たしています。薄い文字は、目立たせたくない補足に使いますが、**読めなくてよい**わけではありません。

##### 解説2: 色を変数にまとめる理由

- 色を `#64748b` のように、あちこちに直接書くと、1つ直すときに全部を探すことになります。変数（`var(--color-muted)`）にしておけば、`:root` の1行を直すだけで、全部の画面が変わります。そして、変えたらこのスクリプトをもう一度動かせば、**読みやすさが崩れていないか**を数で確かめられます（エラー2）。
- RustNote の CSS は、Tailwind のような道具を使わない、ふつうの CSS です。決まりを少なくして、1つのファイル（290 行）で全部の画面をまかなっています（第6巻 UNIT 08 のアセットとして配っています）。

### もう一歩: 実務に近い形

#### 例9-2: 全部のページの HTML を、決まりの表で確かめる

> **▼ このコードがやること（先に日本語で）:** alice を登録してノートを1件作り、6つのページ（トップ・登録・ログイン・一覧・詳細・編集）の HTML を取ります。それぞれについて、`<html lang="ja">` があるか、`<h1>` がちょうど1つか、hidden 以外の入力欄に `<label for="id">` か `aria-label` があるか、ボタンに文字があるかを確かめ、ページごとの `<title>` と、見つかった問題を表示します。

```rust
//! 第9巻 UNIT 09: どのページも、アクセシビリティの最低限の決まりを守っているかを、HTML から確かめる。
//! - <html lang="ja"> がある
//! - <h1> がちょうど1つ
//! - 入力欄（hidden 以外の input・textarea）には、<label for="id"> か aria-label がある
//! - ボタンには、文字がある
//!
//! 実行: cargo test --test u09_a11y -- --nocapture

mod common;

use common::{body_text, test_app};

/// タグの中の、ある属性の値（無ければ None）
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!(" {name}=\"");
    let start = tag.find(&key)? + key.len();
    let end = tag[start..].find('"')? + start;
    Some(&tag[start..end])
}

/// HTML の中の、ある名前のタグ（< から > まで）を全部
fn tags<'a>(html: &'a str, name: &str) -> Vec<&'a str> {
    let open = format!("<{name}");
    let mut found = Vec::new();
    let mut rest = html;
    while let Some(at) = rest.find(&open) {
        let tail = &rest[at..];
        let end = tail.find('>').unwrap_or(tail.len() - 1);
        found.push(&tail[..=end]);
        rest = &tail[end + 1..];
    }
    found
}

/// 1ページを確かめて、見つかった問題を返す
fn check(html: &str) -> Vec<String> {
    let mut problems = Vec::new();
    if !html.contains("<html lang=\"ja\">") {
        problems.push("lang が無い".to_string());
    }
    let h1 = tags(html, "h1").len();
    if h1 != 1 {
        problems.push(format!("<h1> が {h1} 個"));
    }
    for tag in tags(html, "input")
        .into_iter()
        .chain(tags(html, "textarea"))
    {
        if matches!(attr(tag, "type"), Some("hidden" | "submit")) {
            continue;
        }
        let labeled = attr(tag, "aria-label").is_some()
            || attr(tag, "id").is_some_and(|id| html.contains(&format!("<label for=\"{id}\"")));
        if !labeled {
            let name = attr(tag, "name").unwrap_or("?");
            problems.push(format!("ラベルの無い入力欄（name={name}）"));
        }
    }
    for button in html.split("<button").skip(1) {
        let text = button.split("</button>").next().unwrap_or("");
        let inner = text.split_once('>').map(|(_, t)| t.trim()).unwrap_or("");
        if inner.is_empty() {
            problems.push("文字の無いボタン".to_string());
        }
    }
    problems
}

#[tokio::test]
async fn pages_follow_the_basic_rules() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;

    let pages = [
        ("/", None),
        ("/signup", None),
        ("/login", None),
        ("/notes", Some(alice.as_str())),
        ("/notes/1", Some(alice.as_str())),
        ("/notes/1/edit", Some(alice.as_str())),
    ];
    let mut total = 0;
    for (path, cookie) in pages {
        let html = body_text(app.get(path, cookie).await).await;
        let title = html
            .split("<title>")
            .nth(1)
            .and_then(|t| t.split("</title>").next())
            .unwrap_or("");
        let problems = check(&html);
        total += problems.len();
        let result = if problems.is_empty() {
            "ok".to_string()
        } else {
            problems.join("・")
        };
        println!("{path:<14} <title>{title}</title>  {result}");
    }
    println!("見つかった問題: {total} 個");
}
```

```powershell
cargo test --test u09_a11y -- --nocapture
```

```text
### cargo test --test u09_a11y -- --nocapture
/              <title>RustNote</title>  ok
/signup        <title>RustNote</title>  ok
/login         <title>RustNote</title>  ok
/notes         <title>RustNote</title>  ラベルの無い入力欄（name=q）
/notes/1       <title>RustNote</title>  ok
/notes/1/edit  <title>RustNote</title>  ok
見つかった問題: 1 個
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s
```

- 6つのうち5つは `ok` でした。登録・ログイン・ノートの作成と編集のフォームは、どの入力欄にも `<label for="…">` が付いています。
- **一覧（`/notes`）の検索の入力欄（`name=q`）に、ラベルがありません**でした。`placeholder="キーワードで検索"`（入力欄の中の、薄い見本の文字）はありますが、`placeholder` は、文字を入れると消えてしまい、読み上げソフトによっては読まれないので、ラベルの代わりにはなりません。これは RustNote に残っている改善点です（問2で直し方を確かめます。RustNote のコードは変えていません）。
- `<title>` は、6つとも `RustNote` でした。ブラウザのタブにも、読み上げソフトの「今のページ」にも、同じ名前しか出ません。ページごとに「ノートの一覧 - RustNote」のようにすると、もっと分かりやすくなります（問3）。

> **注意:** このテストは、HTML の文字を読む、簡単な確かめです。`<label>` の中に入力欄を入れる書き方（`<label>名前 <input></label>`）や、`aria-labelledby` は見ていません。本格的には、ブラウザで動かす確かめの道具を使います（第11巻 UNIT 05（準備中））。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: ラベルの for と、入力欄の id がずれた

作成と編集のフォーム（`src/views.rs` の `note_form`）で、`<label for="title">` を `<label for="note-title">` に変えてしまった場合です。入力欄の `id="title"` は、そのままです。

```text
### （views.rs の <label for="title"> を <label for="note-title"> にして）cargo test --test u09_a11y -- --nocapture
/              <title>RustNote</title>  ok
/signup        <title>RustNote</title>  ok
/login         <title>RustNote</title>  ok
/notes         <title>RustNote</title>  ラベルの無い入力欄（name=q）・ラベルの無い入力欄（name=title）
/notes/1       <title>RustNote</title>  ok
/notes/1/edit  <title>RustNote</title>  ラベルの無い入力欄（name=title）
見つかった問題: 3 個
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s
```

- **何が起きたか:** 画面の見た目は、何も変わりません。ラベルの文字は、入力欄の上に、ちゃんと表示されています。それでも、テストは、一覧と編集の `name=title` の入力欄を「ラベルの無い入力欄」と報告しました。`for` と `id` がずれると、ラベルと入力欄は、**つながっていない**からです。
- **困ること:** 読み上げソフトは、入力欄に入ったとき、何の欄かを読めません。ラベルの文字を押しても、入力欄に移れません（`for` が合っていれば、ラベルを押すと、その入力欄に移ります）。
- **学べること:** 見た目で確かめられない間違いこそ、テストで確かめます。

#### エラー2（エラーが出ない間違い）: 薄い文字を、もっと薄くした

デザインの都合で、`--color-muted` を、もう少し明るい `#94a3b8` に変えた場合です。

```text
### （--color-muted を #94a3b8 にした CSS で）python contrast.py style_light.css
組み合わせ                              比     AA（4.5 以上）
本文の文字            #0f172a / #f8fafc  17.06  ok
日時などの薄い文字        #94a3b8 / #f8fafc   2.45  NG
カードの中の薄い文字       #94a3b8 / #ffffff   2.56  NG
リンク              #c2410c / #f8fafc   4.95  ok
ボタンの文字           #ffffff / #c2410c   5.18  ok
白いボタン（secondary）の文字 #c2410c / #ffffff   5.18  ok
削除のボタン・エラーの文字    #dc2626 / #ffffff   4.83  ok
成功のお知らせ          #047857 / #ecfdf5   5.21  ok
注意のお知らせ          #b45309 / #fffbeb   4.84  ok
```

- **何が起きたか:** 薄い文字の2つの組み合わせが、**2.45 と 2.56** になり、`NG` です。AA の 4.5 の、半分ほどしかありません。画面を作った人の画面では「すっきりして見える」かもしれませんが、目の弱い人・明るい外でスマートフォンを見る人には、読めない文字になります。
- **直し方:** 色を変えたら、`contrast.py` をもう一度動かし、全部が `ok` の色を選びます。CI で動かしておけば、うっかり変えたときにも気づけます。

### RustNote ではここで使う

- 色は `:root` の 12 個の変数だけで、ボタンは `button`（主）・`.secondary`（副）・`.danger`（危険）の3種類です。詳細ページの「編集」「完了にする」「削除」が、ちょうどこの3種類です（第10巻 UNIT 03（準備中））。
- 入力のエラーは、入力欄のすぐ下の `<p class="field-error">`（`--color-danger`）に、お知らせは、ページの上の `<p class="flash …" role="status">` に出します（第10巻 UNIT 02・第6巻 UNIT 05）。`role="status"` は、読み上げソフトに「この文は、お知らせです」と伝える印です。
- `style.css` には、フォーカスの枠（`outline`）を消す決まりがありません。ブラウザの既定の枠がそのまま出るので、Tab キーで動くと、今どこにいるかが見えます。**`outline: none` を書かない**のも、決まりの1つです。
- ページの外枠（`routes.rs` の `root_layout`）は `<html lang="ja">` と `<meta name="viewport">` を必ず出します。`viewport` は、スマートフォンで、画面の幅に合わせて表示するための設定です。

### 練習問題

**問1:** ボタンにマウスを乗せると、背景が `--color-primary-dark`（`#9a3412`）になります（`button:hover`）。白い文字とのコントラスト比は、いくつでしょうか。

**問2:** 例9-2で見つかった、一覧の検索の入力欄のラベルが無い問題を、作業用のコピーで直し、テストで確かめてください。

**問3:** どのページの `<title>` も `RustNote` でした。ページごとに `<title>` を変えるには、どうすればよいでしょうか。

### 解答と解説

**問1の解答:**

```text
### マウスを乗せたときのボタン（白い文字 / --color-primary-dark）
#ffffff / #9a3412  7.31
```

- 7.31 で、AA（4.5）を超え、AAA（7 以上）も満たします。マウスを乗せたときは、ふだん（5.18）より濃くなるので、読みやすさは上がります。

**問2の解答:**

検索の入力欄に、`aria-label="キーワードで検索"` を足します（`src/routes/notes.rs` の `notes_index`。作業用のコピーで変えた所だけを抜き出しています）。

```rust
            <input
                name="q"
                type="search"
                aria-label="キーワードで検索"
                placeholder="キーワードで検索"
                value=(keyword.as_deref())
            >
```

```text
### （検索の入力欄に aria-label を足して）cargo test --test u09_a11y -- --nocapture
/notes         <title>RustNote</title>  ok
見つかった問題: 0 個
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s
```

- 一覧も `ok` になり、問題は 0 個になりました。検索の入力欄は、すぐ横の「検索」ボタンで意味が分かるので、見える `<label>` を足さずに、読み上げのための `aria-label` だけにしています。見える文字のラベルを置けるなら、`<label for="q">` と `id="q"` を使うほうが、目で見る人にも分かりやすくなります。

**問3の解答:**

- `<title>` は、レイアウト（`root_layout`）の `<head>` に書いてあるので、どのページも同じになります。ページごとに変えるには、ページから題を渡せるようにします。たとえば、ページの `view!` の中で `<title>` を出す部品を作る、ページごとに題を `cx` の値として登録してレイアウトで読む、などの方法があります。
- RustNote は、いまは全ページ同じ題です。題があっても、ページの中の `<h1>`（「ノート」「ログイン」など）が、今どのページかを伝えているので、困り方は小さいですが、ブラウザのタブをいくつも開いたときには、どれがどれか分かりにくくなります。直すときは、`u09_a11y` のテストに「`<title>` が `<h1>` の文字を含む」という決まりを足すと、全ページを一度に確かめられます。

### まとめ

- 見た目の決まりは、色の変数（12 個）とボタンの3種類のように、少なく決めて、意味ごとに1つずつ使う。色は変数にまとめ、変えたらコントラスト比を計算し直す。
- 文字と背景のコントラスト比は、WCAG の AA で 4.5 以上。RustNote の9つの組み合わせは、すべて満たしている（薄い文字は 4.55 でぎりぎり）。
- `lang`・`<h1>` が1つ・入力欄のラベル・ボタンの文字は、本物の HTML をテストで確かめる。見た目では気づけないずれ（`for` と `id`）も見つかる。検索の入力欄のラベルと、ページごとの `<title>` は、RustNote に残る改善点。
