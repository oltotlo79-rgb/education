## UNIT 10: 狭い画面・アクセシビリティ・総合の試験

### この単元のゴール

- この巻の画面を、利用者が使う順番に1本の流れでたどる**総合の試験**を書き、どこかの約束が破れたら、その段の名前で止まるようにできる
- 本物のブラウザ（Chrome）で、幅 320・375・768 ピクセルの画面に、横のはみ出しが無いかを測り、長い英数字のタイトルではみ出すことと、その直し方を確かめられる
- キーボードの Tab だけで、どの順番で移れるか、移った所に枠が見えるか、Enter と Space でボタンが押せるかを、実際に確かめられる
- `outline: none` がキーボードの利用者から何を奪うかと、この巻で見つかった RustNote の改善点を、まとめて説明できる

### たとえ話でつかむ

家が建ったときの、完成の検査を考えます。

- 職人さんは、工事のたびに、**部屋ごとの点検**をしています。水道は水が出るか、窓は閉まるか。これが、UNIT 01〜09 で書いてきた、画面ごとのテストです。
- 完成の検査では、もう1つ、**住む人の動きで、家の中を通しで歩いて**みます。玄関から入り、台所で料理をして、2階で寝て、朝に出かける。1つずつの部屋が大丈夫でも、通して歩くと、「台所の扉を開けると、廊下がふさがる」のような困りごとが見つかります。
- そして、**いろいろな人の目で**歩きます。車いすの人は、狭い廊下を通れるか。暗いところでも、スイッチの場所が分かるか。いつも見ている人の目だけでは、見落とす所があります。

| 家の完成の検査 | RustNote の確かめ |
|---|---|
| 部屋ごとの点検 | UNIT 01〜09 の、画面ごとのテスト |
| 通しで歩いてみる | 総合の試験（`tests/u10_journey.rs`）。登録から退出まで1本の流れ |
| 狭い廊下を通れるか | 幅 320・375 ピクセルの画面で、横にはみ出さないか |
| 暗いところでスイッチが分かるか | キーボードの Tab で移った所に、枠が見えるか |

> **用語:** 総合の試験（ここでは、いくつもの画面を、利用者が使う順番に1本の流れでたどるテスト。E2E（end to end：端から端まで）とも呼ぶ。第11巻 UNIT 05（準備中））

<!-- -->

> **用語:** フォーカス（キーボードの入力を受け取る、画面の中の1つの場所。Tab キーで次へ、Shift+Tab で前へ移る。今どこにあるかを示す枠を、フォーカスの枠と呼ぶ）

<!-- -->

> **用語:** 横のはみ出し（画面の幅より中身が広くて、横に動かさないと全部が見えないこと。スマートフォンでは、とても読みにくくなる）

### 図で理解する

この巻の確かめの、3つの層です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="226" height="230" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="123" y="36" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">画面ごとのテスト</text>
<text x="123" y="56" font-size="11" fill="#475569" text-anchor="middle">UNIT 01〜09（cargo test）</text>
<text x="30" y="88" font-size="11" fill="#0f172a">・1つの約束を、くわしく</text>
<text x="30" y="110" font-size="11" fill="#0f172a">・わざと壊して、気づくか</text>
<text x="30" y="132" font-size="11" fill="#0f172a">・速い（1つ 1 秒くらい）</text>
<text x="30" y="176" font-size="11" fill="#475569">見つけにくい物:</text>
<text x="30" y="196" font-size="11" fill="#475569">画面と画面のつなぎ目</text>
<rect x="247" y="10" width="226" height="230" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">総合の試験</text>
<text x="360" y="56" font-size="11" fill="#475569" text-anchor="middle">例10-1（cargo test）</text>
<text x="267" y="88" font-size="11" fill="#0f172a">・登録から退出まで1本で</text>
<text x="267" y="110" font-size="11" fill="#0f172a">・段ごとに名前の付いた assert</text>
<text x="267" y="132" font-size="11" fill="#0f172a">・壊れたら、その段で止まる</text>
<text x="267" y="176" font-size="11" fill="#475569">見つけにくい物:</text>
<text x="267" y="196" font-size="11" fill="#475569">見た目・キーボードでの使い方</text>
<rect x="484" y="10" width="226" height="230" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="597" y="36" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ブラウザでの確かめ</text>
<text x="597" y="56" font-size="11" fill="#475569" text-anchor="middle">例10-2（Chrome の Console）</text>
<text x="504" y="88" font-size="11" fill="#0f172a">・幅 320・375・768 で測る</text>
<text x="504" y="110" font-size="11" fill="#0f172a">・Tab の順番と、枠が見えるか</text>
<text x="504" y="132" font-size="11" fill="#0f172a">・Enter と Space で押せるか</text>
<text x="504" y="176" font-size="11" fill="#475569">見つけにくい物:</text>
<text x="504" y="196" font-size="11" fill="#475569">毎回の自動の確かめ（人が動かす）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: この巻の確かめの3つの層。画面ごとのテストで約束をくわしく、総合の試験でつなぎ目を、ブラウザで見た目とキーボードでの使い方を確かめる</div>
</div>

詳細の画面を、キーボードの Tab だけで移ったときの順番です（例10-2で、実際に確かめた結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="44" rx="6" fill="#ffffff" stroke="#e2e8f0" stroke-width="2"/>
<rect x="24" y="18" width="100" height="28" rx="4" fill="none" stroke="#0f172a" stroke-width="2"/>
<text x="74" y="37" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">① RustNote</text>
<text x="430" y="37" font-size="12" fill="#c2410c" text-anchor="middle">② ノート一覧</text>
<text x="545" y="37" font-size="11" fill="#64748b" text-anchor="middle">alice@example.com</text>
<text x="650" y="37" font-size="12" fill="#c2410c" text-anchor="middle">③ ログアウト</text>
<rect x="150" y="70" width="420" height="80" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="2"/>
<text x="170" y="98" font-size="15" font-weight="700" fill="#0f172a">週末の予定</text>
<text x="170" y="120" font-size="10" fill="#64748b">作成: … ／ 更新: …</text>
<text x="170" y="140" font-size="11" fill="#0f172a">土曜日は図書館で本を返して、…</text>
<rect x="150" y="162" width="70" height="30" rx="6" fill="#c2410c"/>
<text x="185" y="182" font-size="12" fill="#ffffff" text-anchor="middle">④ 編集</text>
<rect x="228" y="162" width="104" height="30" rx="6" fill="#ffffff" stroke="#c2410c" stroke-width="2"/>
<text x="280" y="182" font-size="12" fill="#c2410c" text-anchor="middle">⑤ 完了にする</text>
<rect x="340" y="162" width="70" height="30" rx="6" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
<text x="375" y="182" font-size="12" fill="#dc2626" text-anchor="middle">⑥ 削除</text>
<text x="150" y="218" font-size="12" fill="#c2410c">⑦ ← 一覧に戻る</text>
<text x="600" y="110" font-size="11" fill="#475569" text-anchor="middle">見出しや本文は</text>
<text x="600" y="128" font-size="11" fill="#475569" text-anchor="middle">Tab では止まらない</text>
<text x="600" y="146" font-size="11" fill="#475569" text-anchor="middle">（押せる物だけ）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 詳細の画面の Tab の順番。押せる物（リンク・ボタン）だけに、HTML に書いた順で止まる。①の枠のように、止まった所にはブラウザの枠が出る</div>
</div>

### まず動かす最小コード

#### 例10-1: 総合の試験 — 登録から退出まで1本でたどる

> **▼ このコードがやること（先に日本語で）:** alice の登録から、空の一覧・作成・空のタイトルの 400・エスケープされた詳細・`%` の検索・編集・古い version の 409・完了の切り替え・bob から見えないこと・削除・ログアウトまで、12 の段を順にたどります。段ごとに `assert` で「こうなるはず」を確かめ、通ったら `✓` と段の名前を表示します。約束が破れた段があれば、その段の名前を付けて止まります。

```rust
//! 第10巻 UNIT 10: 総合の試験。この巻の画面を、利用者が使う順番に1本の流れでたどり、
//! 段ごとに「こうなるはず」を assert で確かめる。どこかで約束が破れたら、その段の名前で止まる。
//! 実行: cargo test --test u10_journey -- --nocapture

mod common;

use common::{TestApp, body_text, encode, location, test_app};
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

/// notes 表の件数
async fn count(app: &TestApp) -> i64 {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await.unwrap();
    rows.next().await.unwrap().unwrap().get(0).unwrap()
}

/// 1つの段を表示する
fn ok(step: &str) {
    println!("✓ {step}");
}

#[tokio::test]
async fn whole_journey() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    ok("1. alice が新規登録できる");

    let html = body_text(app.get("/notes", Some(&alice)).await).await;
    assert!(
        html.contains("まだノートがありません"),
        "2. 最初の一覧は空のはず"
    );
    ok("2. 最初の一覧は空で、次にすることを伝える");

    let to = app.create_note(&alice, "<b>牛乳</b>を買う", "1本").await;
    assert_eq!(to, "/notes/1", "3. 作成したら詳細へ移るはず");
    ok("3. 作成すると 303 で詳細へ移る");

    let r = app.post_form("/notes", "title=&body=", Some(&alice)).await;
    assert_eq!(
        r.status(),
        StatusCode::BAD_REQUEST,
        "4. 空のタイトルは 400 のはず"
    );
    assert_eq!(count(&app).await, 1, "4. 400 のときは保存しないはず");
    ok("4. 空のタイトルは 400 で、保存しない");

    let html = body_text(app.get("/notes/1", Some(&alice)).await).await;
    assert!(
        html.contains("&lt;b&gt;牛乳&lt;/b&gt;を買う"),
        "5. タイトルはエスケープされるはず"
    );
    ok("5. 詳細では、タイトルの < > が文字として出る");

    let html = body_text(
        app.get(&format!("/notes?q={}", encode("牛乳")), Some(&alice))
            .await,
    )
    .await;
    assert!(html.contains("全1件"), "6. 牛乳で1件見つかるはず");
    let html = body_text(
        app.get(&format!("/notes?q={}", encode("%")), Some(&alice))
            .await,
    )
    .await;
    assert!(html.contains("全0件"), "6. % はただの文字として探すはず");
    ok("6. 検索は、% をただの文字として探す");

    let form = format!("version=1&title={}&body=", encode("牛乳を2本買う"));
    let r = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    assert_eq!(
        r.status(),
        StatusCode::SEE_OTHER,
        "7. 新しい version の保存は 303 のはず"
    );
    ok("7. 編集を保存すると 303");

    let r = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    assert_eq!(
        r.status(),
        StatusCode::CONFLICT,
        "8. 古い version の保存は 409 のはず"
    );
    ok("8. 古い version で保存すると 409");

    let r = app.post_form("/notes/1/toggle", "", Some(&alice)).await;
    assert_eq!(location(&r), "/notes/1", "9. 切り替えたら詳細へ戻るはず");
    let html = body_text(app.get("/notes/1", Some(&alice)).await).await;
    assert!(html.contains("未完了に戻す"), "9. 完了になっているはず");
    ok("9. 完了を切り替えると、ボタンが「未完了に戻す」になる");

    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let r = app.get("/notes/1", Some(&bob)).await;
    assert_eq!(
        r.status(),
        StatusCode::NOT_FOUND,
        "10. bob には alice のノートが 404 のはず"
    );
    let r = app.post_form("/notes/1/delete", "", Some(&bob)).await;
    assert_eq!(
        r.status(),
        StatusCode::NOT_FOUND,
        "10. bob は alice のノートを消せないはず"
    );
    ok("10. bob には、alice のノートが見えず、消せない");

    let r = app.post_form("/notes/1/delete", "", Some(&alice)).await;
    assert_eq!(location(&r), "/notes", "11. 削除したら一覧へ移るはず");
    assert_eq!(count(&app).await, 0, "11. 削除したら 0 件のはず");
    ok("11. alice が削除すると、一覧へ移り、0 件になる");

    let r = app.post_form("/logout", "", Some(&alice)).await;
    assert_eq!(
        location(&r),
        "/login",
        "12. ログアウトしたらログイン画面へ移るはず"
    );
    let r = app.get("/notes", Some(&alice)).await;
    assert_eq!(
        location(&r),
        "/login",
        "12. 古いクッキーでは一覧を開けないはず"
    );
    ok("12. ログアウトすると、古いクッキーでは一覧を開けない");
}
```

```powershell
cargo test --test u10_journey -- --nocapture
```

```text
✓ 1. alice が新規登録できる
✓ 2. 最初の一覧は空で、次にすることを伝える
✓ 3. 作成すると 303 で詳細へ移る
✓ 4. 空のタイトルは 400 で、保存しない
✓ 5. 詳細では、タイトルの < > が文字として出る
✓ 6. 検索は、% をただの文字として探す
✓ 7. 編集を保存すると 303
✓ 8. 古い version で保存すると 409
✓ 9. 完了を切り替えると、ボタンが「未完了に戻す」になる
✓ 10. bob には、alice のノートが見えず、消せない
✓ 11. alice が削除すると、一覧へ移り、0 件になる
✓ 12. ログアウトすると、古いクッキーでは一覧を開けない
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.96s
```

（段の行と、結果の行だけを抜き出しています）

#### ▼ コードを1つずつ分解して解説

##### 解説1: 12 の段が、この巻の UNIT に対応している

- 2は UNIT 01（空のとき）、3と4は UNIT 02（PRG と 400）、5は UNIT 03（詳細とエスケープ）、6は UNIT 07（検索）、7と8は UNIT 04（編集と 409）、9は UNIT 05（完了）、10は UNIT 03 と UNIT 06（他人のノート）、11は UNIT 06（削除）、12はお知らせとログアウト（UNIT 09）です。
- 1本の流れなので、**前の段の結果を、次の段が使います**。たとえば8は、7で `version` が 2 になったあとで、同じ `version=1` のフォームを送るので 409 になります。画面ごとのテストでは、毎回、新しい DB から始めるので、こうした「つなぎ目」は確かめにくいのです。

##### 解説2: assert に、段の名前を付ける

- `assert_eq!(r.status(), StatusCode::CONFLICT, "8. 古い version の保存は 409 のはず")` のように、3つ目の引数に「どの段の、何の約束か」を書いています。止まったとき、テストの名前（`whole_journey`）だけでは、長い流れのどこで止まったか分かりません（エラー3）。
- 総合の試験は、細かいことを確かめすぎないようにします。何十もの `assert` を1本に詰めると、どこかを少し変えるたびに止まり、直すのが大変になります。くわしい確かめは、画面ごとのテストに任せます。

### もう一歩: 実務に近い形

#### 例10-2: 本物のブラウザで、狭い画面とキーボードを確かめる

ここからは、`cargo run` で RustNote を起動し、Chrome で `http://127.0.0.1:3000/` を開いて、alice でログインした状態で確かめます。ノートは、普通の題の3件と、空白の無い長い英数字の題（`https://example.com/notes/2026/10/30/a-very-long-address-without-any-spaces`、80 文字）の1件を作っておきます。

確かめには、Chrome の**開発者ツール**（F12 キー）の **Console**（コンソール：JavaScript を1行ずつ動かせる場所）を使います。下のコードを貼り付けて Enter を押すと、結果が表示されます。

##### 狭い画面: はみ出しを測る

> **▼ このコードがやること（先に日本語で）:** 今のページの中に、幅を決めた枠（`iframe`）を作り、RustNote のページを開いて、ページ全体の横幅（`scrollWidth`）が枠の幅を超えていないかを測ります。超えていたら、中身が箱より広くなっている要素を表示します。4つのページを、幅 320・375・768 ピクセルで測ります。

```javascript
// 狭い画面の確かめ: 幅を決めた iframe でページを開き、横にはみ出していないかを測る
const probe = async (path, width) => {
  const f = document.createElement('iframe');
  f.style.cssText = `width:${width}px;height:900px;border:0`;
  f.src = path;
  document.body.appendChild(f);
  await new Promise(r => f.onload = r);
  const d = f.contentDocument;
  const page = d.documentElement.scrollWidth;
  const worst = [...d.body.querySelectorAll('*')]
    .filter(e => e.scrollWidth > e.clientWidth + 1 && e.clientWidth > 0)
    .map(e => `${e.tagName.toLowerCase()} ${e.clientWidth}→${e.scrollWidth}px`).pop();
  f.remove();
  return `${path.padEnd(14)} ${width}px: 横幅 ${page}px ${page > width ? `× はみ出す（${worst}）` : '○'}`;
};
const lines = [];
for (const w of [320, 375, 768]) for (const p of ['/notes', '/notes/2', '/notes/3', '/notes/3/edit']) lines.push(await probe(p, w));
lines.join('\n')
```

```text
/notes         320px: 横幅 332px × はみ出す（li 286→315px）
/notes/2       320px: 横幅 320px ○
/notes/3       320px: 横幅 537px × はみ出す（h1 246→500px）
/notes/3/edit  320px: 横幅 320px ○
/notes         375px: 横幅 375px ○
/notes/2       375px: 横幅 375px ○
/notes/3       375px: 横幅 537px × はみ出す（h1 301→500px）
/notes/3/edit  375px: 横幅 375px ○
/notes         768px: 横幅 768px ○
/notes/2       768px: 横幅 768px ○
/notes/3       768px: 横幅 768px ○
/notes/3/edit  768px: 横幅 768px ○
```

- 普通の題のページ（`/notes/2`）と編集の画面は、3つの幅とも `○` でした。本文の枠（`.container`）は `max-width: 760px` と左右 16 ピクセルの余白で、狭い画面では画面の幅いっぱいに縮みます。一覧の1行は `flex-wrap: wrap` なので、入りきらない部品は、次の行へ回る作りです。
- **長い英数字の題の詳細（`/notes/3`）は、320 でも 375 でもはみ出しました**。`h1` の箱は 301 ピクセルなのに、中身が 500 ピクセルあります。空白の無い英数字は、ブラウザが途中で折り返せる場所が少ないからです。一覧（`/notes`）も、幅 320 では、1行の箱（286 ピクセル）より題が広くなりました（エラー1）。
- 768（タブレットくらい）では、どのページもはみ出しませんでした。

##### キーボード: Tab の順番と、枠

> **▼ このコードがやること（先に日本語で）:** ページの中でフォーカスが移るたびに（`focusin`）、移った先の要素の種類・名前（ラベルか、中の文字）・フォーカスの枠の形（`outlineStyle`）を、`focusLog` に記録します。このコードを動かしたあと、ページの何も無い所を1回クリックしてから、Tab キーを押していき、最後に `focusLog` を表示します。

```javascript
window.focusLog = [];
document.addEventListener('focusin', (e) => {
  const el = e.target;
  const tag = el.tagName.toLowerCase() + (el.name ? ` name=${el.name}` : '');
  const name = el.labels?.[0]?.textContent ?? el.textContent.trim();
  const ring = getComputedStyle(el).outlineStyle;
  window.focusLog.push(`${tag}「${name}」 枠: ${ring}`);
});
// Tab を押したあとで:
focusLog.map((t, i) => `${i + 1}. ${t}`).join('\n')
```

詳細の画面（`/notes/2`）で、Tab を7回押した結果です。

```text
1. a「RustNote」 枠: auto
2. a「ノート一覧」 枠: auto
3. button「ログアウト」 枠: auto
4. a「編集」 枠: auto
5. button「完了にする」 枠: auto
6. button「削除」 枠: auto
7. a「← 一覧に戻る」 枠: auto
```

一覧の画面（`/notes`）で、Tab を6回押して「新しいノートを書く」で Enter を押し、さらに Tab を3回押した結果です（1行目は、そのときの `details` が開いているか）。

```text
details open=true
1. a「RustNote」 枠: auto
2. a「ノート一覧」 枠: auto
3. button「ログアウト」 枠: auto
4. input name=q「」 枠: auto
5. button「検索」 枠: auto
6. summary「新しいノートを書く」 枠: auto
7. input name=title「タイトル（80文字まで）」 枠: auto
8. textarea name=body「本文（10000文字まで）」 枠: auto
9. button「作成する」 枠: auto
```

- Tab は、押せる物（リンク・ボタン・入力欄・`summary`）だけに、HTML に書いた順で止まりました。見出しや本文には止まりません。順番は、見た目の上から下・左から右と同じです。RustNote は、`tabindex` で順番を変えていないからです。
- どこに止まっても、枠は `auto`（ブラウザの決まりの枠）でした。`style.css` に `outline: none` が無いので、ブラウザの枠がそのまま出ます（第9巻 UNIT 09）。
- 「新しいノートを書く」（`<summary>`）は、Enter で開き（`details open=true`）、続く Tab で、ラベルの付いたタイトル・本文の入力欄と「作成する」に移れました。閉じているときは、中の入力欄には止まりません。
- 一覧の検索の入力欄（`input name=q`）の名前は `「」`（空）でした。ラベルが無いからです。第9巻 UNIT 09 で HTML から見つけた改善点を、ブラウザでも確かめたことになります。

##### キーボード: Enter と Space で押す

Tab で「完了にする」へ移って Enter を、続けて「未完了に戻す」へ移って Space を押し、そのあとの画面を Console で読みました。

```javascript
// 1回目（Enter のあと）は active も読んだ。2回目（Space のあと）は active を外した
({url: location.href, flash: document.querySelector('.flash')?.textContent, badge: !!document.querySelector('.badge'), buttons: [...document.querySelectorAll('.note-actions button')].map(b => b.textContent.trim()), active: document.activeElement.tagName})
```

```text
（Tab で「完了にする」へ移り、Enter を押したあと）
{
  "active": "BODY",
  "badge": true,
  "buttons": [
    "未完了に戻す",
    "削除"
  ],
  "flash": "完了の状態を切り替えました",
  "url": "http://127.0.0.1:3000/notes/2"
}
（Tab で「未完了に戻す」へ移り、Space を押したあと）
{
  "badge": false,
  "buttons": [
    "完了にする",
    "削除"
  ],
  "flash": "完了の状態を切り替えました",
  "url": "http://127.0.0.1:3000/notes/2"
}
```

- Enter でも Space でも、ボタンが押され、完了の状態が切り替わって、お知らせが出ました。`<button>` は、何も書かなくても、キーボードで押せます。`<div>` にクリックの処理を付けた「ボタンもどき」は、Tab で止まらず、Enter でも押せません。RustNote は、押せる物を全部 `<a>` と `<button>` で作っているので、この心配がありません。
- 押したあとのフォーカスは `BODY`（ページの先頭）に戻りました。PRG で画面を読み直したからです。キーボードの利用者は、もう一度 Tab を押して進みます。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 長い英数字の題が、狭い画面ではみ出す

これは、**今の RustNote で起きる**はみ出しです（例10-2）。`style.css` は、本文（`.note-body`）にだけ `overflow-wrap: anywhere`（どこででも折り返してよい）を付けていて、題（`h1` と、一覧の題のリンク）には付けていません。そこで、ブラウザの中でだけ、`<style>` で1行足して測り直しました（RustNote のファイルは変えていません）。

```javascript
const probe = async (path, width, css) => {
  const f = document.createElement('iframe');
  f.style.cssText = `width:${width}px;height:900px;border:0`;
  f.src = path;
  document.body.appendChild(f);
  await new Promise(r => f.onload = r);
  const d = f.contentDocument;
  if (css) { const s = d.createElement('style'); s.textContent = css; d.head.appendChild(s); }
  const page = d.documentElement.scrollWidth;
  const h1 = d.querySelector('h1');
  const res = `${path.padEnd(9)} ${width}px: 横幅 ${page}px ${page > width ? '× はみ出す' : '○'}（h1 の高さ ${Math.round(h1.getBoundingClientRect().height)}px）`;
  f.remove();
  return res;
};
const fix = 'h1, .note-list a { overflow-wrap: anywhere; }';
const lines = ['--- いまの CSS'];
for (const w of [320, 375]) for (const p of ['/notes', '/notes/3']) lines.push(await probe(p, w, null));
lines.push(`--- 足した CSS: ${fix}`);
for (const w of [320, 375]) for (const p of ['/notes', '/notes/3']) lines.push(await probe(p, w, fix));
lines.join('\n')
```

```text
--- いまの CSS
/notes    320px: 横幅 332px × はみ出す（h1 の高さ 44px）
/notes/3  320px: 横幅 537px × はみ出す（h1 の高さ 133px）
/notes    375px: 横幅 375px ○（h1 の高さ 44px）
/notes/3  375px: 横幅 537px × はみ出す（h1 の高さ 133px）
--- 足した CSS: h1, .note-list a { overflow-wrap: anywhere; }
/notes    320px: 横幅 320px ○（h1 の高さ 44px）
/notes/3  320px: 横幅 320px ○（h1 の高さ 177px）
/notes    375px: 横幅 375px ○（h1 の高さ 44px）
/notes/3  375px: 横幅 375px ○（h1 の高さ 177px）
```

- **何が起きたか:** 1行足すと、320 でも 375 でも、はみ出しが無くなりました。題の `h1` は、高さが 133 から 177 ピクセルに増えています。横に収まらない分を、行を増やして縦に並べたのです。
- **気づきにくい理由:** 作る人が、普通の題・広い画面で見ている限り、何も起きません。サーバーにも、テストにも、エラーは出ません。URL をそのまま題にする人や、狭いスマートフォンを使う人のところでだけ、起きます。
- **直し方:** `assets/style.css` に `h1, .note-list a { overflow-wrap: anywhere; }` を足し、`topcoat asset bundle` で束を作り直します。これは、RustNote に残る改善点です。

#### エラー2（エラーが出ない間違い）: outline: none で、フォーカスの枠を消した

「クリックしたときに出る枠が、見た目によくない」と考えて、CSS に `:focus { outline: none; }` を足した場合です。ブラウザの中でだけ `<style>` を足し、Tab で「完了にする」へ移って、枠の形を読みました。

```javascript
const st = document.createElement('style');
st.textContent = ':focus { outline: none; }';
document.head.appendChild(st);
// Tab で「完了にする」へ移ってから（足す前にも、同じ2行で読んだ）:
const el = document.activeElement; const s = getComputedStyle(el);
`${el.textContent.trim()}: outline ${s.outlineStyle} ${s.outlineWidth} ${s.outlineColor}`
```

```text
（いまの CSS）完了にする: outline auto 1px rgb(16, 16, 16)
（:focus { outline: none; } を足したあと）完了にする: outline none 3px rgb(194, 65, 12)
```

- **何が起きたか:** いまの CSS では `outline auto 1px`（ブラウザの決まりの枠）でしたが、足したあとは `outline none` になり、画面のボタンには、何の印も出なくなりました。フォーカスは確かに「完了にする」にあるので、Enter を押せば押せます。でも、**今どこにいるのか**が、見えません。
- **困ること:** マウスを使わない（使えない）人は、Tab を押すたびに、どこにいるか分からないまま、Enter を押すことになります。間違って「削除」を押してしまうかもしれません（問1）。
- **学べること:** 枠の見た目を変えたいときは、消さずに、`:focus-visible { outline: 3px solid var(--color-primary); }` のように、**別の見える枠に置き換えます**。`:focus-visible` は、主にキーボードで移ったときに枠を出す指定なので、ボタンをクリックしたときの見た目も気になりません。

#### エラー3: 総合の試験が、段の名前で止まる

総合の試験は、どこかの約束が破れたときに、その段の名前で止まります。この巻で試した間違いのうち2つを、練習用のコピーで入れ直して動かしました（どちらも、確かめたら元に戻します）。まず、UNIT 04 のエラー1（`UPDATE` から version の条件を外す）です。

```text
✓ 1. alice が新規登録できる
✓ 2. 最初の一覧は空で、次にすることを伝える
✓ 3. 作成すると 303 で詳細へ移る
✓ 4. 空のタイトルは 400 で、保存しない
✓ 5. 詳細では、タイトルの < > が文字として出る
✓ 6. 検索は、% をただの文字として探す
✓ 7. 編集を保存すると 303
thread 'whole_journey' (212604) panicked at tests\u10_journey.rs:81:5:
assertion `left == right` failed: 8. 古い version の保存は 409 のはず
  left: 303
 right: 409
```

次に、UNIT 03 のエラー1（`find` の SQL から持ち主の条件を外す）です。

```text
✓ 1. alice が新規登録できる
✓ 2. 最初の一覧は空で、次にすることを伝える
✓ 3. 作成すると 303 で詳細へ移る
✓ 4. 空のタイトルは 400 で、保存しない
✓ 5. 詳細では、タイトルの < > が文字として出る
✓ 6. 検索は、% をただの文字として探す
✓ 7. 編集を保存すると 303
✓ 8. 古い version で保存すると 409
✓ 9. 完了を切り替えると、ボタンが「未完了に戻す」になる
thread 'whole_journey' (217256) panicked at tests\u10_journey.rs:96:5:
assertion `left == right` failed: 10. bob には alice のノートが 404 のはず
  left: 200
 right: 404
```

- **何が起きたか:** 1つ目は、`7` まで `✓` が並び、**`8. 古い version の保存は 409 のはず`** で止まりました（`left: 303`・`right: 409`）。2つ目は、`9` まで通って、**`10. bob には alice のノートが 404 のはず`** で止まりました（`left: 200`・`right: 404`）。
- **読み方:** `✓` の行で、どこまで約束が守られていたかが分かり、止まった段の名前で、どの約束が破れたかが分かります。テストのファイルの行（81 行目・96 行目）を開けば、送った内容も分かります。
- 段の名前を付けずに `assert_eq!(r.status(), StatusCode::CONFLICT)` とだけ書くと、止まったときに出るのは `left: 303`・`right: 409` だけで、12 の段のどこなのかを、行の番号から探すことになります。

### RustNote ではここで使う

- ページの外枠（`src/routes.rs` の `root_layout`）は、`<meta name="viewport" content="width=device-width, initial-scale=1">` を出しています。スマートフォンのブラウザに「画面の幅に合わせて表示して」と伝える指定です（第9巻 UNIT 09）。
- 狭い画面に合わせる仕組みは、`.container` の `max-width: 760px` と余白、一覧の1行の `flex-wrap: wrap`、詳細の本文の `white-space: pre-wrap` と `overflow-wrap: anywhere` です。`@media`（画面の幅で見た目を切り替える指定）はありません。
- 押せる物は、全部 `<a>` と `<button>` と `<summary>` です。`tabindex` も、`outline: none` もありません。
- この巻で見つけた、RustNote に残る改善点を、まとめておきます（どれも、RustNote のコードは変えていません）。

| 見つけた所 | 改善点 |
|---|---|
| UNIT 04 の問2 | 409 のお知らせがタイトルだけで、本文だけの変更に気づけない |
| UNIT 06 | 削除に、確かめの画面も、元に戻す仕組みも無い |
| UNIT 08 の問1 | 範囲の外のページで「まだノートがありません」と出る |
| UNIT 10 のエラー1 | 長い英数字の題が、狭い画面で横にはみ出す |
| 第9巻 UNIT 09・UNIT 10 | 一覧の検索の入力欄に、ラベルが無い |

### 練習問題

**問1:** キーボードの Tab で詳細の画面の「削除」へ移り、Enter を押すと、どうなるでしょうか。

**問2:** 一覧と編集の画面で、Tab が止まらなかった入力欄があります。どれでしょうか。止まらないのは、困ることでしょうか。

**問3:** 総合の試験に入れるとよいこと・入れないほうがよいことを、1つずつ挙げてください。

### 解答と解説

**問1の解答:**

例10-2の確かめの途中で、実際に起きたことです（確かめ用のノートが1件消えました）。

```text
（Tab で「削除」へ移り、Enter を押したあと）
{
  "flash": "ノートを削除しました",
  "titles": [
    "完了したノート",
    "https://example.com/notes/2026/10/30/a-very-long-address-without-any-spaces",
    "週末の予定"
  ],
  "url": "http://127.0.0.1:3000/notes"
}
（RustNote の窓のアクセスログ）
POST /notes/1/delete -> 303 See Other (10.0015ms)
```

- 確かめの画面も無く、**すぐに消えました**。一覧へ移り、「ノートを削除しました」のお知らせが出て、「牛乳を買う」が無くなっています。サーバーのログにも `POST /notes/1/delete -> 303` があります。
- `<button>` は、キーボードでもマウスと同じように押せるので、押し間違いも、そのまま削除になります。Tab の位置を1つ数え間違えるだけで起きます。エラー2のように、フォーカスの枠が見えなければ、もっと起きやすくなります。UNIT 06 の改善点（確かめの画面か、元に戻す仕組み）は、キーボードの利用者にとっても大事です。

**問2の解答:**

- 編集の画面の hidden の `version`（UNIT 04）と、閉じている「新しいノートを書く」の中の入力欄です。例10-2の Tab の順番に、どちらも出てきません。
- hidden の入力欄は、画面に見えないので、止まらないのが正しい動きです。閉じた `details` の中も、見えない間は止まらず、開けば止まります（一覧の 7〜9）。どちらも困りません。困るのは、見えているのに止まらない物（`<div>` で作ったボタンもどきなど）で、RustNote にはありません。

**問3の解答:**

- **入れるとよいこと:** 利用者が必ず通る流れと、画面と画面のつなぎ目です。たとえば「作成 → 移動先の詳細 → 編集 → 古いフォームの 409」のように、前の画面の結果が次の画面に効く所です。どの段で止まったかが分かるように、段の名前を付けます。
- **入れないほうがよいこと:** 細かい決まり（タイトルの 80 文字と 81 文字の境目、LIKE の記号の1つずつ、など）です。それは画面ごとのテストや、ユニットテストで確かめます（UNIT 02・UNIT 07・第11巻 UNIT 01）。総合の試験に詰め込むと、1本が長くなり、どこかを変えるたびに止まって、直すのが大変になります。
- 見た目とキーボードは、cargo test では確かめにくいので、例10-2のようにブラウザで確かめます。毎回自動で行うには、ブラウザを動かす道具を使います（第11巻 UNIT 05（準備中））。

### まとめ

- 総合の試験は、登録から退出まで、利用者が使う順番に1本でたどり、段ごとに名前の付いた `assert` で確かめる。壊れたら、`✓` の続きの、止まった段の名前で、どの約束が破れたかが分かる。
- 本物のブラウザで、幅 320・375・768 のはみ出しを測ると、長い英数字の題が 320・375 ではみ出した（改善点）。`h1, .note-list a { overflow-wrap: anywhere; }` を足すと、はみ出しは無くなる。
- Tab は押せる物だけに HTML の順で止まり、枠は消さない。Enter と Space で `<button>` が押せる。`outline: none` は、キーボードの利用者から「今どこにいるか」を奪う。削除がキーボードでもすぐに動くので、確かめの画面か元に戻す仕組みが要る。
