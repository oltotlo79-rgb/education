## UNIT 02: リポジトリの結合テスト

### この単元のゴール

- リポジトリ（`NoteRepository` など）を、偽物ではなく**本物の DB**（一時フォルダの libSQL ファイル）で確かめる理由を、SQL の細かい決まりの例で説明できる
- 準備（Arrange）・操作（Act）・確かめ（Assert）の形で、成功・見つからない・他人・版の食い違いを、1つずつ確かめるテストを書ける
- わざと SQL を壊したとき、ユニットテストは通ったまま、結合テストだけが気づく様子を確かめられる
- テストどうしが DB ファイルを使い回したときに起きる、実行のたびに変わる失敗を読み、テストごとに新しい DB を作る形に直せる

### たとえ話でつかむ

レストランの厨房で、新しいスープの味を確かめる場面を考えます。

- UNIT 01 のユニットテストは、**材料1つずつの味見**です。「塩はこの量」「だしはこの濃さ」と、1つずつ確かめます。速くて、どこが悪いかもすぐ分かります。
- ところが、材料が全部正しくても、**実際の鍋で煮る**と、火加減や、入れる順番のせいで、思った味にならないことがあります。そこで、本物の鍋で煮て味見をします。これが、この単元の**結合テスト**です。
- 味見のたびに、**洗った新しい鍋**を使います。前の人が使った鍋のままだと、前のスープの味が残っていて、どちらの味なのか分からなくなります。2人が同時に1つの鍋を使えば、もっと混乱します。

| 厨房 | RustNote のテスト |
|---|---|
| 材料1つずつの味見 | ユニットテスト（`escape_like` や `NoteTitle::parse` を、DB なしで確かめる。UNIT 01） |
| 本物の鍋で煮て味見 | リポジトリの結合テスト（本物の libSQL に、本物の SQL を流す） |
| 火加減・入れる順番 | SQL の細かい決まり（`LIKE` の `ESCAPE`・`WHERE` の条件・`UNIQUE`・`ON DELETE CASCADE`） |
| 洗った新しい鍋 | テストごとに作る一時フォルダの DB（`common::test_db()`） |
| 前のスープの味が残る | 前のテストのデータが残った DB。結果が実行のたびに変わる |

> **用語:** 結合テスト（けつごうテスト：いくつかの部品を、本物どうしでつないで確かめるテスト。ここでは、リポジトリと本物の DB をつなぐ）

<!-- -->

> **用語:** Arrange・Act・Assert（アレンジ・アクト・アサート：テストを「準備する」「操作する」「確かめる」の3つに分けて書く形）

### 図で理解する

テストの段と、それぞれが確かめるものです。この単元は、下から2段目です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="200" height="44" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="120" y="47" font-size="12" fill="#475569" text-anchor="middle">E2E（UNIT 05）</text>
<rect x="20" y="74" width="200" height="44" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="120" y="101" font-size="12" fill="#475569" text-anchor="middle">HTTP のテスト（UNIT 03）</text>
<rect x="20" y="128" width="200" height="52" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="3"/>
<text x="120" y="152" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">リポジトリの結合テスト</text>
<text x="120" y="170" font-size="11" fill="#1e40af" text-anchor="middle">（この UNIT 02）</text>
<rect x="20" y="190" width="200" height="44" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="120" y="217" font-size="12" fill="#475569" text-anchor="middle">ユニットテスト（UNIT 01）</text>
<text x="250" y="47" font-size="11" fill="#475569">ブラウザで、画面を操作する</text>
<text x="250" y="101" font-size="11" fill="#475569">ルーター・状態コード・クッキー・HTML</text>
<text x="250" y="150" font-size="12" font-weight="700" fill="#0f172a">本物の SQL を、本物の libSQL で動かす</text>
<text x="250" y="170" font-size="11" fill="#0f172a">WHERE の持ち主の条件・LIKE の ESCAPE・UNIQUE・版の番号・CASCADE</text>
<text x="250" y="217" font-size="11" fill="#475569">Rust の関数1つ（DB なし）</text>
<text x="700" y="47" font-size="11" fill="#c2410c" text-anchor="end">遅い・少なく</text>
<text x="700" y="217" font-size="11" fill="#047857" text-anchor="end">速い・たくさん</text>
<line x1="690" y1="200" x2="690" y2="62" stroke="#94a3b8" stroke-width="2"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: テストの段。リポジトリの結合テストは、ユニットテストでは確かめられない「SQL そのもの」を、本物の DB で確かめる</div>
</div>

テストごとに、新しい DB を用意します。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-0201" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">RustNote の形（テストごとに新しい DB）</text>
<rect x="20" y="36" width="140" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="90" y="61" font-size="12" fill="#0f172a" text-anchor="middle">テスト A</text>
<rect x="200" y="36" width="140" height="40" rx="6" fill="#f8fafc" stroke="#10b981" stroke-width="2"/>
<text x="270" y="54" font-size="11" fill="#0f172a" text-anchor="middle">一時フォルダ 1</text>
<text x="270" y="69" font-size="11" fill="#475569" text-anchor="middle">test.db（空・最新の表）</text>
<rect x="20" y="96" width="140" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="90" y="121" font-size="12" fill="#0f172a" text-anchor="middle">テスト B</text>
<rect x="200" y="96" width="140" height="40" rx="6" fill="#f8fafc" stroke="#10b981" stroke-width="2"/>
<text x="270" y="114" font-size="11" fill="#0f172a" text-anchor="middle">一時フォルダ 2</text>
<text x="270" y="129" font-size="11" fill="#475569" text-anchor="middle">test.db（空・最新の表）</text>
<line x1="160" y1="56" x2="196" y2="56" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0201)"/>
<line x1="160" y1="116" x2="196" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0201)"/>
<text x="180" y="166" font-size="11" fill="#0f172a" text-anchor="middle">同時に動いても、ぶつからない</text>
<text x="180" y="184" font-size="11" fill="#0f172a" text-anchor="middle">テストが終わると、フォルダごと消える</text>
<text x="540" y="22" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">間違えた形（1つのファイルを使い回す）</text>
<rect x="380" y="36" width="120" height="40" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="440" y="61" font-size="12" fill="#0f172a" text-anchor="middle">テスト A</text>
<rect x="380" y="96" width="120" height="40" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="440" y="121" font-size="12" fill="#0f172a" text-anchor="middle">テスト B</text>
<rect x="560" y="60" width="150" height="56" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="635" y="82" font-size="11" fill="#0f172a" text-anchor="middle">shared-test.db</text>
<text x="635" y="100" font-size="11" fill="#475569" text-anchor="middle">前の実行のデータも残る</text>
<line x1="500" y1="56" x2="556" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0201)"/>
<line x1="500" y1="116" x2="556" y2="98" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0201)"/>
<text x="545" y="166" font-size="11" fill="#b91c1c" text-anchor="middle">同時に書いて、ぶつかる（UNIQUE・locked）</text>
<text x="545" y="184" font-size="11" fill="#b91c1c" text-anchor="middle">結果が、実行のたびに変わる（エラー2）</text>
<line x1="360" y1="30" x2="360" y2="200" stroke="#e2e8f0" stroke-width="2"/>
<text x="360" y="232" font-size="12" fill="#0f172a" text-anchor="middle">テストの結果は「そのテストの中でしたこと」だけで決まるようにする</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: テストごとの DB。RustNote の common::test_db() は、テストごとに一時フォルダを作り、マイグレーションを当てた空の DB を渡す</div>
</div>

### まず動かす最小コード

#### 例2-1: NoteRepository の約束を、1つずつ確かめる

`NoteRepository` の6つのメソッドには、それぞれ約束があります（`src/repository/notes.rs` のコメント）。たとえば「他人のノート・存在しないノートは None」「`expected_version` が今の version と違えば `VersionMismatch`」です。これを、本物の DB で1つずつ確かめます。

> **▼ このコードがやること（先に日本語で）:** 空の DB に alice と bob を登録します（準備）。alice のノートを作り、探す・更新する・完了にする・消す、を、alice と bob の両方から試します（操作）。それぞれの結果を、1行ずつ表示します（確かめ）。ここでは結果を見るために表示していますが、本物のテストでは `assert!` で確かめます。

```rust
//! 第11巻 UNIT 02: NoteRepository の約束を、本物の DB（一時フォルダの libSQL ファイル）で1つずつ確かめる。
//! 実行: cargo test --test u02_contract -- --nocapture

mod common;

use rustnote::{
    domain::{Email, NewNote, Note, NoteId},
    error::RepoError,
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

/// ノートの大事な所だけを、1行の文字列にする
fn brief(result: Result<Note, RepoError>) -> String {
    match result {
        Ok(note) => format!(
            "Ok（id={} version={} done={} title={:?}）",
            note.id, note.version, note.done, note.title
        ),
        Err(error) => format!("Err({error:?})"),
    }
}

fn new_note(title: &str) -> NewNote {
    NewNote::parse(title, "").expect("テスト用のノート")
}

#[tokio::test]
async fn note_repository_contract() {
    // Arrange: 空の DB と、2人のユーザー
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = users
        .create(
            &Email::parse("alice@example.com").unwrap(),
            "$argon2id$dummy",
        )
        .await
        .unwrap();
    let bob = users
        .create(&Email::parse("bob@example.com").unwrap(), "$argon2id$dummy")
        .await
        .unwrap();

    // Act と Assert: 約束を1つずつ
    let created = notes.insert(alice.id, &new_note("初版")).await.unwrap();
    let id = created.id;
    println!(
        "insert（alice）             → {}",
        brief(Ok(created.clone()))
    );
    println!(
        "   作成と更新の日時が同じ   → {}",
        created.created_at == created.updated_at
    );

    let found = notes.find(alice.id, id).await.unwrap();
    println!(
        "find（alice）               → 見つかる: {}",
        found.is_some()
    );
    let found = notes.find(bob.id, id).await.unwrap();
    println!(
        "find（bob）                 → 見つかる: {}",
        found.is_some()
    );

    let result = notes.update(alice.id, id, &new_note("第2版"), 1).await;
    println!("update（alice, version 1）  → {}", brief(result));
    let result = notes.update(alice.id, id, &new_note("古い画面"), 1).await;
    println!("update（alice, version 1）  → {}", brief(result));
    let missing = NoteId::new(99).unwrap();
    let result = notes.update(alice.id, missing, &new_note("無い"), 1).await;
    println!("update（alice, id 99）      → {}", brief(result));
    let result = notes.update(bob.id, id, &new_note("乗っ取り"), 2).await;
    println!("update（bob, version 2）    → {}", brief(result));

    let result = notes.toggle_done(alice.id, id).await;
    println!("toggle_done（alice）        → {}", brief(result));

    println!(
        "delete（bob）               → {:?}",
        notes.delete(bob.id, id).await
    );
    println!(
        "delete（alice）             → {:?}",
        notes.delete(alice.id, id).await
    );
    println!(
        "delete（alice, もう一度）   → {:?}",
        notes.delete(alice.id, id).await
    );
}
```

コピーした `rustnote` フォルダの `tests/u02_contract.rs` として保存して実行します。

```powershell
cargo test --test u02_contract -- --nocapture
```

**▼ 実行結果**

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running tests\u02_contract.rs (target\debug\deps\u02_contract-28783a9a9988e5a0.exe)

running 1 test
insert（alice）             → Ok（id=1 version=1 done=false title="初版"）
   作成と更新の日時が同じ   → true
find（alice）               → 見つかる: true
find（bob）                 → 見つかる: false
update（alice, version 1）  → Ok（id=1 version=2 done=false title="第2版"）
update（alice, version 1）  → Err(VersionMismatch)
update（alice, id 99）      → Err(NotFound)
update（bob, version 2）    → Err(NotFound)
toggle_done（alice）        → Ok（id=1 version=3 done=true title="第2版"）
delete（bob）               → Err(NotFound)
delete（alice）             → Ok(())
delete（alice, もう一度）   → Err(NotFound)
test note_repository_contract ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 準備は、毎回まっさらな DB から

- `common::test_db().await` が、新しい一時フォルダに DB を作り、マイグレーション（表を作る SQL。第13巻 UNIT 02）を当てて渡します。だから、最初のノートの id は、いつも 1 です。
- ユーザーは `LibsqlUserRepository::create` で登録します。パスワードのハッシュは、ここでは関係ないので `"$argon2id$dummy"` という仮の文字列です。

##### 解説2: 成功だけでなく、断られる場合を全部試す

- `update` は4回呼びました。正しい version（1）なら成功して version 2 に、同じ version 1 でもう一度なら `VersionMismatch`（版の食い違い。別のタブで先に更新された）、無い id 99 なら `NotFound`、bob からなら `NotFound` です。
- bob からの更新は、version を正しい 2 にしても `NotFound` でした。「他人のノートは、あっても無いのと同じ」という約束です（第8巻 UNIT 07）。`VersionMismatch` にすると、「そのノートはある」と bob に教えてしまいます。
- 削除は、bob から → `NotFound`、alice から → `Ok(())`、もう一度 → `NotFound` です。1回目と3回目の `NotFound` は、理由は違います（他人／もう無い）が、外から見ると同じです。

##### 解説3: 本物の DB でも、テストは速い

- 実行結果の最後の行のとおり、このテストは数十ミリ秒で終わります。一時フォルダのファイルなので、ネットワークの待ち時間がありません。RustNote の `tests/repository.rs` の8つのテストも、まとめて 1 秒かかりません。
- 速いので、「本物の DB は遅いから偽物にする」必要は、ここではありません。

### もう一歩: 実務に近い形

#### 例2-2: SQL を1か所壊すと、どのテストが気づくか

本物の DB で確かめると、何が良いのでしょうか。SQL をわざと1か所壊して、確かめます。試すときは、練習用のコピーの `src/repository/notes.rs` を書き換え、確かめたら必ず元に戻してください。

検索の SQL の、`ESCAPE '\'`（`\` のうしろの文字を、ただの文字として扱う、という指定）を消します。

```rust
        // 元: "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')"
        let condition = "user_id = ?1 AND (title LIKE ?2 OR body LIKE ?2)";
```

まず、`escape_like` のユニットテスト（`src/repository/notes.rs` の中の `like_special_characters_are_escaped`）を動かします。

```powershell
cargo test --lib like_special
```

```text

running 1 test
test repository::notes::tests::like_special_characters_are_escaped ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s
```

ユニットテストは**通りました**。次に、リポジトリの結合テストを動かします。

```powershell
cargo test --test repository
```

```text

running 8 tests
test migrations_are_applied_only_once ... ok
test duplicate_email_is_a_unique_violation ... ok
test update_uses_optimistic_locking ... ok
test deleting_a_session_logs_out ... ok
test sessions_expire ... ok
test notes_are_isolated_per_owner ... ok
test toggle_and_delete ... ok
test search_filters_by_keyword_and_paginates ... FAILED

failures:

---- search_filters_by_keyword_and_paginates stdout ----

thread 'search_filters_by_keyword_and_paginates' (215108) panicked at tests\repository.rs:198:5:
assertion `left == right` failed
  left: 0
 right: 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    search_filters_by_keyword_and_paginates

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

error: test failed, to rerun pass `--test repository`
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-0202" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="40" width="130" height="50" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="85" y="62" font-size="11" fill="#475569" text-anchor="middle">キーワード</text>
<text x="85" y="80" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">100%</text>
<rect x="190" y="40" width="160" height="50" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="270" y="62" font-size="11" fill="#047857" text-anchor="middle">escape_like（Rust）</text>
<text x="270" y="80" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">%100\%%</text>
<rect x="390" y="20" width="310" height="60" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="545" y="42" font-size="11" fill="#047857" text-anchor="middle">LIKE ?2 ESCAPE '<tspan font-family="Consolas, monospace">\</tspan>'（SQL・元のまま）</text>
<text x="545" y="64" font-size="12" fill="#0f172a" text-anchor="middle">「100%」を含む → 1 件</text>
<rect x="390" y="100" width="310" height="60" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="545" y="122" font-size="11" fill="#b91c1c" text-anchor="middle">LIKE ?2（ESCAPE を消した）</text>
<text x="545" y="144" font-size="12" fill="#0f172a" text-anchor="middle">「100<tspan font-family="Consolas, monospace">\</tspan>」を含む → 0 件</text>
<line x1="150" y1="65" x2="186" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0202)"/>
<line x1="350" y1="60" x2="386" y2="50" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0202)"/>
<line x1="350" y1="72" x2="386" y2="128" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0202)"/>
<text x="270" y="122" font-size="11" fill="#047857" text-anchor="middle">ユニットテストが</text>
<text x="270" y="140" font-size="11" fill="#047857" text-anchor="middle">確かめるのは、ここまで</text>
<text x="360" y="190" font-size="12" fill="#0f172a" text-anchor="middle">作った模様（Rust）が正しくても、SQL がその模様をどう読むかは、本物の DB でしか確かめられない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: ESCAPE を消したときの違い。escape_like の結果は同じでも、SQL が <span style="font-family:Consolas,monospace;">\</span> を「ただの <span style="font-family:Consolas,monospace;">\</span>」として読むので、何も見つからなくなる</div>
</div>

- **何が起きたか:** 8つのテストのうち、`search_filters_by_keyword_and_paginates` だけが失敗しました。198 行目は `assert_eq!(percent.total_count, 1);` で、「100%」で検索した件数が、1 のはずが 0 でした。
- **なぜ 0 件か:** `escape_like` は、`100%` を `100\%` に書き換えます（これは正しいままです）。ところが SQL に `ESCAPE '\'` が無いと、`\` は「次の文字をただの文字にする印」ではなく、**ただの `\`** として読まれます。だから「`100\` を含むノート」を探してしまい、1件も見つかりません。
- **学べること:** Rust の関数（`escape_like`）と、SQL（`ESCAPE '\'`）は、**2つそろって**初めて正しく動きます。ユニットテストは片方しか見ていないので、もう片方が壊れても気づけません。偽物のリポジトリ（第4巻 UNIT 02）でも、SQL は動かないので気づけません。**SQL を書いたら、本物の DB で動かすテストを置く**のが、この単元の中心です。確かめたら、`src/repository/notes.rs` を元に戻します。

### よくあるエラーと読み方

#### エラー1: `assertion failed: matches!(…)` だけで、何が返ったか分からない

今度は、削除の SQL から、持ち主の条件 `AND user_id = ?2` を消してみます（練習用のコピーで試し、必ず元に戻します）。

```rust
                "DELETE FROM notes WHERE id = ?1",
```

```powershell
cargo test --test repository
```

```text

running 8 tests
test migrations_are_applied_only_once ... ok
test duplicate_email_is_a_unique_violation ... ok
test deleting_a_session_logs_out ... ok
test update_uses_optimistic_locking ... ok
test sessions_expire ... ok
test toggle_and_delete ... ok
test notes_are_isolated_per_owner ... FAILED
test search_filters_by_keyword_and_paginates ... ok

failures:

---- notes_are_isolated_per_owner stdout ----

thread 'notes_are_isolated_per_owner' (218972) panicked at tests\repository.rs:90:5:
assertion failed: matches!(notes.delete(bob.id, note.id).await, Err(RepoError::NotFound))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    notes_are_isolated_per_owner

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

error: test failed, to rerun pass `--test repository`
```

- **何が起きたか:** `notes_are_isolated_per_owner` が、90 行目で失敗しました。bob が alice のノートを消そうとしたら `NotFound` になるはずが、そうならなかった、という意味です。テストが無ければ、**誰でも、他人のノートを id だけで消せる**ところでした（IDOR。第8巻 UNIT 01）。
- **気づきにくい点:** SQL の `?2` を消したのに、`params![id, owner]` で2つの値を渡したままでも、エラーにはなりませんでした。libsql は、使われない値を黙って無視しました。**コンパイルも、実行も通る**ので、テストだけが頼りです。
- **困る点:** `assertion failed: matches!(…)` は、**実際に何が返ったか**を表示しません。`Ok(())` だったのか、別のエラーだったのか、この表示だけでは分かりません。

**直し方:** `assert!` の2つ目の引数に、失敗したときの文と、実際の値を書いておきます（`tests/u02_owner_delete.rs`）。

```rust
//! 第11巻 UNIT 02 のエラー1の直し方: 失敗したときに「実際の値」が出る形で確かめる。
//! 実行: cargo test --test u02_owner_delete

mod common;

use rustnote::{
    domain::{Email, NewNote},
    error::RepoError,
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

#[tokio::test]
async fn other_user_cannot_delete() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = users
        .create(
            &Email::parse("alice@example.com").unwrap(),
            "$argon2id$dummy",
        )
        .await
        .unwrap();
    let bob = users
        .create(&Email::parse("bob@example.com").unwrap(), "$argon2id$dummy")
        .await
        .unwrap();
    let note = notes
        .insert(alice.id, &NewNote::parse("買い物", "").unwrap())
        .await
        .unwrap();

    let result = notes.delete(bob.id, note.id).await;
    // 2つ目の引数に、失敗したときに出す文と、実際の値を書いておく
    assert!(
        matches!(result, Err(RepoError::NotFound)),
        "bob の削除は NotFound のはずが、{result:?} だった"
    );
    let still_there = notes.find(alice.id, note.id).await.unwrap();
    assert!(still_there.is_some(), "alice のノートが消えている");
}
```

壊したままの SQL で動かすと、次のように表示されます。

```powershell
cargo test --test u02_owner_delete
```

```text

running 1 test
test other_user_cannot_delete ... FAILED

failures:

---- other_user_cannot_delete stdout ----

thread 'other_user_cannot_delete' (215892) panicked at tests\u02_owner_delete.rs:35:5:
bob の削除は NotFound のはずが、Ok(()) だった
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    other_user_cannot_delete

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

error: test failed, to rerun pass `--test u02_owner_delete`
```

- 「bob の削除は NotFound のはずが、`Ok(())` だった」と、実際の値が出ました。「bob の削除が**成功してしまった**」と、すぐに分かります。
- 元の SQL に戻すと、このテストは通ります（`test other_user_cannot_delete ... ok`）。**失敗したときに読む人**のために、確かめる所に一言書いておくのが、良いテストです。

#### エラー2: テストが、同じ DB ファイルを使い回した

テストごとに DB を作るのが面倒で、決まった1つのファイル（`rustnote-shared-test.db`）を、どのテストからも開くようにした場合です（`tests/u02_shared.rs`）。2つのテストは、どちらも alice を登録します。

```rust
//! 第11巻 UNIT 02 のエラー2: 2つのテストが、同じ DB ファイルを使い回したとき。
//! 実行: cargo test --test u02_shared

use rustnote::{
    db::{self, Db},
    domain::Email,
    repository::{LibsqlUserRepository, UserRepository},
};

/// どのテストも、決まった1つのファイルを開いてしまう（一時フォルダを作らない）
async fn shared_db() -> Db {
    let path = std::env::temp_dir().join("rustnote-shared-test.db");
    let db = Db::open_local(path).await.expect("DB を開けません");
    db::migrate(&db).await.expect("マイグレーションに失敗");
    db
}

#[tokio::test]
async fn sign_up_alice() {
    let users = LibsqlUserRepository::new(shared_db().await);
    let email = Email::parse("alice@example.com").unwrap();
    users.create(&email, "$argon2id$dummy").await.unwrap();
}

#[tokio::test]
async fn alice_can_be_found() {
    let users = LibsqlUserRepository::new(shared_db().await);
    let email = Email::parse("alice@example.com").unwrap();
    users.create(&email, "$argon2id$dummy").await.unwrap();
    assert!(users.find_by_email(&email).await.unwrap().is_some());
}
```

ファイルが無い状態から、1回目を動かします。

```powershell
cargo test --test u02_shared
```

```text

running 2 tests
test sign_up_alice ... ok
test alice_can_be_found ... FAILED

failures:

---- alice_can_be_found stdout ----

thread 'alice_can_be_found' (219836) panicked at tests\u02_shared.rs:29:51:
called `Result::unwrap()` on an `Err` value: UniqueViolation("SQLite failure: `UNIQUE constraint failed: users.email`")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    alice_can_be_found

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

error: test failed, to rerun pass `--test u02_shared`
```

- **何が起きたか:** `sign_up_alice` は通り、`alice_can_be_found` が `UniqueViolation`（同じメールアドレスが、もうある）で失敗しました。2つのテストは**同時に**動くので、先に alice を登録したほうが通り、あとのほうが失敗します。どちらが先かは、その時しだいです。

同じことを、ファイルを消してから 20 回くり返すと、毎回1つは通り、もう1つの失敗の理由は、次のように変わりました。

| 失敗の理由 | 回数（20 回のうち） | どこで止まったか |
|---|---|---|
| `UniqueViolation`（alice がもういる） | 17 回 | alice を登録するところ |
| `database is locked`（ほかの接続が書いている最中） | 3 回 | DB を開くところ |

```text
thread 'alice_can_be_found' (215524) panicked at tests\u02_shared.rs:13:41:
DB を開けません: SqliteFailure(5, "database is locked")
```

- この 20 回では出ませんでしたが、前の版（Topcoat 0.10・Rust 1.99.0）で8回試したときには、マイグレーションのところで `table notes already exists`（表がもうある）で止まったことも1回ありました。

- 2つのテストが、同じファイルを**同時に**開いて、同時にマイグレーションを当てようとするので、「表を作る」「書き込む」がぶつかります。どこでぶつかるかは、毎回違います。

ファイルを消さずに、もう一度動かします。

```text

running 2 tests
test sign_up_alice ... FAILED
test alice_can_be_found ... FAILED

failures:

---- sign_up_alice stdout ----

thread 'sign_up_alice' (218360) panicked at tests\u02_shared.rs:22:51:
called `Result::unwrap()` on an `Err` value: UniqueViolation("SQLite failure: `UNIQUE constraint failed: users.email`")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- alice_can_be_found stdout ----

thread 'alice_can_be_found' (218476) panicked at tests\u02_shared.rs:29:51:
called `Result::unwrap()` on an `Err` value: UniqueViolation("SQLite failure: `UNIQUE constraint failed: users.email`")


failures:
    alice_can_be_found
    sign_up_alice

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

error: test failed, to rerun pass `--test u02_shared`
```

- 今度は、**両方とも**失敗しました。前の実行で登録した alice が、ファイルに残っているからです。
- **直し方:** RustNote のテストのように、`common::test_db()` で、テストごとに新しい一時フォルダの DB を作ります。テストの結果が「そのテストの中でしたこと」だけで決まるようにすると、同時に動かしても、何回動かしても、同じ結果になります。確かめたら、`tests/u02_shared.rs` と、一時フォルダ（PowerShell なら `$env:TEMP`）にできた `rustnote-shared-test.db` で始まるファイルを消しておきます。
- **学べること:** 「ときどき失敗するテスト」は、テストどうしが何かを**共有している**ことが多い原因です。DB のファイル・決まったポート番号・環境変数・今の時刻などです。失敗したら、まず「このテストは、ほかのテストと何を共有しているか」を探します。

### RustNote ではここで使う

テストごとの DB は、`tests/common/mod.rs` の `test_db` が作ります（抜き出しています）。

```rust
/// 一時フォルダの DB。TempDir が消えるとフォルダごと削除される
pub struct TestDb {
    pub db: Db,
    _dir: TempDir,
}

/// マイグレーション済みの空の DB を作る
pub async fn test_db() -> TestDb {
    let dir = tempfile::tempdir().expect("一時フォルダを作れません");
    let db = Db::open_local(dir.path().join("test.db"))
        .await
        .expect("DB を開けません");
    db::migrate(&db).await.expect("マイグレーションに失敗");
    TestDb { db, _dir: dir }
}
```

- `tempfile::tempdir()` は、ほかと重ならない名前の一時フォルダを作ります。テストが同時にいくつ動いても、それぞれ別のフォルダです。
- `TestDb` は、`db` と一緒に `TempDir` を持っています。テストが終わって `TestDb` が片付けられる（drop される）と、`TempDir` もフォルダごと消えます（第3巻 UNIT 01）。項目の名前が `_dir` と `_` で始まるのは、「使わないが、持っておくこと自体に意味がある」という印です。`TempDir` を持っておくのは、テストの間はフォルダを残し、終わったら消すためです。
- 本番と同じ `db::migrate` を当てるので、テストの DB の表は、いつも本番と同じ形です。マイグレーションを足せば、テストの DB にも自動で入ります。

`tests/repository.rs` は、この `test_db` を使って、リポジトリの約束を8つのテストで確かめています。ユーザーを登録する手順は、何度も出てくるので、小さな関数にまとめています（抜き出しています）。

```rust
/// テスト用のユーザーを登録する
async fn create_user(users: &LibsqlUserRepository, email: &str) -> User {
    let email = Email::parse(email).unwrap();
    users.create(&email, "$argon2id$dummy").await.unwrap()
}
```

```powershell
cargo test --test repository
```

```text

running 8 tests
test migrations_are_applied_only_once ... ok
test duplicate_email_is_a_unique_violation ... ok
test update_uses_optimistic_locking ... ok
test deleting_a_session_logs_out ... ok
test sessions_expire ... ok
test notes_are_isolated_per_owner ... ok
test toggle_and_delete ... ok
test search_filters_by_keyword_and_paginates ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

- 8つのテストは、同時に動いて、0.2 秒ほどで終わりました。どれも自分の DB を持っているので、同時に動いてもぶつかりません。

### 練習問題

**問1:** タイトル `Rust 入門`・本文 `SQLite の LIKE` のノートと、タイトル `Äpfel kaufen` のノートを作り、`Rust`・`rust`・`RUST`・`ｒｕｓｔ`（全角）・`sqlite`・`Äpfel`・`äpfel` で検索して、それぞれ何件見つかるかを表示してください。検索は、大文字と小文字を区別するでしょうか。

**問2:** セッションの期限を、決まった時刻 T にして作り、`find_user` を「T の1秒前」「T ちょうど」「T の1秒後」で呼んでください。`delete_expired` も「T の1秒前」と「T ちょうど」で呼び、何件消えたかを表示してください。期限ちょうどの時刻は、まだ使えるでしょうか。

**問3:** alice と bob に、ノートを2件ずつとセッションを1つずつ作ってから、SQL で alice を `users` 表から消し、`users`・`notes`・`sessions` の行の数が、消す前と後でどう変わるかを表示してください。

### 解答と解説

**問1の解答:**

`tests/u02_q1.rs` です。

```rust
//! 問1の解答: 検索（LIKE）は、大文字と小文字を区別するかを確かめる。
//! 実行: cargo test --test u02_q1 -- --nocapture

mod common;

use rustnote::{
    domain::{Email, NewNote, NoteQuery},
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

#[tokio::test]
async fn like_and_letter_case() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let alice = users
        .create(
            &Email::parse("alice@example.com").unwrap(),
            "$argon2id$dummy",
        )
        .await
        .unwrap();
    for (title, body) in [("Rust 入門", "SQLite の LIKE"), ("Äpfel kaufen", "")] {
        notes
            .insert(alice.id, &NewNote::parse(title, body).unwrap())
            .await
            .unwrap();
    }

    for keyword in [
        "Rust",
        "rust",
        "RUST",
        "ｒｕｓｔ",
        "sqlite",
        "Äpfel",
        "äpfel",
    ] {
        let page = notes
            .search(alice.id, &NoteQuery::new(Some(keyword), None))
            .await
            .unwrap();
        println!("{keyword:<8} → {} 件", page.total_count);
    }
}
```

```powershell
cargo test --test u02_q1 -- --nocapture
```

```text

running 1 test
Rust     → 1 件
rust     → 1 件
RUST     → 1 件
ｒｕｓｔ     → 0 件
sqlite   → 1 件
Äpfel    → 1 件
äpfel    → 0 件
test like_and_letter_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

- `Rust`・`rust`・`RUST` は、どれも 1 件でした。SQLite の `LIKE` は、**英字（ASCII）の大文字と小文字を区別しません**。本文にだけある `sqlite` も見つかりました（タイトルと本文の両方を探す SQL だからです）。
- ところが、全角の `ｒｕｓｔ` は 0 件、`äpfel` も 0 件でした。SQLite の `LIKE` が大文字と小文字を同じに扱うのは、ASCII の英字だけで、`Ä` と `ä` のような文字は、別の文字として比べます。全角と半角も、別の文字です。
- これは、RustNote の仕様として決めておくことです。今の RustNote は「英字は大文字と小文字を区別しない。それ以外は、書いたとおりに探す」です。仕様を変えるなら、このテストの表を先に書き換えてから、SQL を直します。

**問2の解答:**

`tests/u02_q2.rs` です。

```rust
//! 問2の解答: セッションの期限ちょうどの時刻で、探す・掃除するとどうなるかを確かめる。
//! 実行: cargo test --test u02_q2 -- --nocapture

mod common;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rustnote::{
    domain::Email,
    repository::{
        LibsqlSessionRepository, LibsqlUserRepository, SessionRepository, UserRepository,
    },
};
use topcoat::session::TokenHash;

#[tokio::test]
async fn session_expiry_boundary() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let sessions = LibsqlSessionRepository::new(test.db.clone());
    let alice = users
        .create(
            &Email::parse("alice@example.com").unwrap(),
            "$argon2id$dummy",
        )
        .await
        .unwrap();

    // 期限（T）を、決まった時刻にしておく（今の時刻に頼らない）
    let t = UNIX_EPOCH + Duration::from_secs(2_000_000_000);
    let second = Duration::from_secs(1);
    let hash = TokenHash::new([7; 32]);
    sessions.create(&hash, alice.id, t).await.unwrap();

    for (label, now) in [
        ("T - 1秒", t - second),
        ("T ちょうど", t),
        ("T + 1秒", t + second),
    ] {
        let found = sessions.find_user(&hash, now).await.unwrap();
        println!("find_user（{label}）→ 見つかる: {}", found.is_some());
    }
    let before: SystemTime = t - second;
    println!(
        "delete_expired（T - 1秒）→ {} 件消した",
        sessions.delete_expired(before).await.unwrap()
    );
    println!(
        "delete_expired（T ちょうど）→ {} 件消した",
        sessions.delete_expired(t).await.unwrap()
    );
}
```

```powershell
cargo test --test u02_q2 -- --nocapture
```

```text

running 1 test
find_user（T - 1秒）→ 見つかる: true
find_user（T ちょうど）→ 見つかる: false
find_user（T + 1秒）→ 見つかる: false
delete_expired（T - 1秒）→ 0 件消した
delete_expired（T ちょうど）→ 1 件消した
test session_expiry_boundary ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

- 期限ちょうどの T では、もう見つかりませんでした。`find_user` の SQL は `s.expires_at > ?2`（期限が今より**あと**なら有効）で、`delete_expired` は `expires_at <= ?1`（期限が今か、今より**前**なら消す）です。T ちょうどは「使えない」側と「消す」側の両方に入るので、すき間も重なりもありません。
- 境界の「ちょうど」は、UNIT 01 の境界値と同じで、いちばん間違えやすい所です。`>` と `>=` を取り違えると、「期限ちょうどの1秒だけ、使えないのに消されない」セッションができます。
- 時刻を `SystemTime::now()` ではなく、決まった時刻（`UNIX_EPOCH` から 20 億秒）にしたのは、テストの結果を、動かした時刻によって変えないためです（エラー2の「共有しているもの」の1つが、今の時刻です）。

**問3の解答:**

`tests/u02_q3.rs` です。

```rust
//! 問3の解答: ユーザーを消すと、その人のノートとセッションはどうなるかを確かめる。
//! 実行: cargo test --test u02_q3 -- --nocapture

mod common;

use std::time::{Duration, SystemTime};

use rustnote::{
    db::Db,
    domain::{Email, NewNote},
    repository::{
        LibsqlNoteRepository, LibsqlSessionRepository, LibsqlUserRepository, NoteRepository,
        SessionRepository, UserRepository,
    },
};
use topcoat::session::TokenHash;

/// 表ごとの行の数を、1行の文字列にする
async fn counts(db: &Db) -> String {
    let conn = db.connect().await.expect("DB に接続できません");
    let mut parts = Vec::new();
    for table in ["users", "notes", "sessions"] {
        let mut rows = conn
            .query(&format!("SELECT COUNT(*) FROM {table}"), ())
            .await
            .expect("数えられません");
        let row = rows
            .next()
            .await
            .expect("行を読めません")
            .expect("行がない");
        let count: i64 = row.get(0).expect("COUNT(*)");
        parts.push(format!("{table}={count}"));
    }
    parts.join(" ")
}

#[tokio::test]
async fn deleting_a_user_cascades() {
    let test = common::test_db().await;
    let users = LibsqlUserRepository::new(test.db.clone());
    let notes = LibsqlNoteRepository::new(test.db.clone());
    let sessions = LibsqlSessionRepository::new(test.db.clone());
    let expires = SystemTime::now() + Duration::from_secs(60);
    for (n, email) in ["alice@example.com", "bob@example.com"]
        .into_iter()
        .enumerate()
    {
        let user = users
            .create(&Email::parse(email).unwrap(), "$argon2id$dummy")
            .await
            .unwrap();
        for title in ["1件目", "2件目"] {
            notes
                .insert(user.id, &NewNote::parse(title, "").unwrap())
                .await
                .unwrap();
        }
        let hash = TokenHash::new([n as u8; 32]);
        sessions.create(&hash, user.id, expires).await.unwrap();
    }
    println!("消す前: {}", counts(&test.db).await);

    // RustNote には「退会」の機能がまだ無いので、SQL で直接 alice を消す
    let conn = test.db.connect().await.unwrap();
    let deleted = conn
        .execute("DELETE FROM users WHERE email = 'alice@example.com'", ())
        .await
        .unwrap();
    println!("users から {deleted} 行消した");
    println!("消した後: {}", counts(&test.db).await);
}
```

```powershell
cargo test --test u02_q3 -- --nocapture
```

```text

running 1 test
消す前: users=2 notes=4 sessions=2
users から 1 行消した
消した後: users=1 notes=2 sessions=1
test deleting_a_user_cascades ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

- `users` から alice の1行を消すと、alice のノート2件とセッション1つも、一緒に消えました。bob の分は残っています。
- `migrations` の表の定義で、`notes.user_id` と `sessions.user_id` に `REFERENCES users(id) ON DELETE CASCADE`（参照先が消えたら、一緒に消す）と書いてあり、libSQL の接続では外部キーの検査が有効（`foreign_keys=1`。第13巻 UNIT 02）だからです。
- もし外部キーの検査が無効だったら、持ち主のいないノートが残ります。こうした**表の約束**も、SQL と同じで、本物の DB でしか確かめられません。RustNote に「退会」の機能を作るときは、このテストを、その機能のテストに育てます。

### まとめ

- リポジトリは、本物の DB（一時フォルダの libSQL ファイル）で確かめる。Rust の関数が正しくても、SQL の `ESCAPE` や `WHERE` の持ち主の条件が壊れていれば、ユニットテストは通ったままになる。
- 準備・操作・確かめの形で、成功だけでなく、見つからない・他人・版の食い違いを1つずつ確かめる。`assert!` には、失敗したときの文と実際の値を書いておく。
- テストごとに新しい DB を作り、テストどうしで何も共有しない。共有すると、結果が実行のたびに変わり、前の実行のデータにも引きずられる。
