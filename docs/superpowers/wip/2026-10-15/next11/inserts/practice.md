## 練習問題（解答・解説つき）

この章の内容が身に付いたかを確かめましょう。まず自分で考えてから、後ろの「解答と解説」を読んでください。Claude Code が手元にある人は、実際に書籍管理アプリで試すとより理解が深まります。

### 問題1: あいまいな指示を書き直す（0.1.2）

次の指示は、Claude Code に意図が伝わりにくい例です。「トップページ（`/`）の書籍一覧の上に、登録されている本の冊数を『全 3 冊』のように表示したい」という意図が伝わるよう、図11-3 の4要素（目的・場所と範囲・完成条件・確かめ方）を入れて書き直してください。

```text
一覧ページをいい感じにして
```

### 問題2: 権限ルールの判定を予想する（2-2・2-3）

`.claude/settings.json` が次のとき、Manual（`default`）モードで Claude Code が (a)〜(d) のコマンドを実行しようとしたら、それぞれどうなりますか？ 「確認なしで実行」「確認が出る」「ブロック」のどれかで答えてください。

```json
{
  "permissions": {
    "allow": ["Bash(npm run *)", "Bash(git commit *)"],
    "ask": ["Bash(git commit *)"],
    "deny": ["Bash(npm run deploy)"]
  }
}
```

- (a) `npm run lint`
- (b) `npm run deploy`
- (c) `git commit -m "feat: 並べ替えを追加"`
- (d) `npm install zod`

### 問題3: 報告と差分が合わない（0.4.1・9-8）

`sortBooks.ts` とそのテストだけを作るよう頼んだところ、Claude Code から「並べ替え機能を実装し、テストも通りました」と報告がありました。`git status --short` の結果は次のとおりです。あなたは次に何を確かめ、どう対処しますか？

```text
 M src/lib/sortBooks.test.ts
 M src/lib/supabase/server.ts
?? src/components/SortSelect.tsx
?? src/lib/sortBooks.ts
```

（`sortBooks.test.ts` は、前回のコミットで一度作ってあったものとします。）

### 問題4: どの機能に書く？（1-4）

次の (a)〜(e) をかなえるには、CLAUDE.md・Rules・Skills・Hooks・Permissions のどれを使うのが最も適切ですか？

- (a) 「部品（コンポーネント）は `src/components/` に1ファイル1つ置く」という決まりを、どの作業でも守ってほしい
- (b) Claude がファイルを編集するたびに、必ず Prettier で整形したい
- (c) `.env.local` を、どんな場合でも Claude に読ませたくない
- (d) 「lint・型チェック・テストを実行して結果を表にまとめる」という作業を、`/` で始まる短い命令で何度も頼みたい
- (e) `src/app/api/` の中のファイルを触るときだけ守ってほしい決まりがある

### 問題5: 計画に頼んでいない変更が入っている（4-1）

プランモードで「`sortBooks` 関数とテストを作る」計画を頼んだところ、計画の「変更するファイル」に次の行が含まれていました。承認画面ではどの選択肢を選び、何と伝えますか？

```text
3. src/app/layout.tsx（変更）: ヘッダーのデザインを新しくする
```

### 問題6: どこまで戻す？（0.4.2）

次の (a)〜(c) の場面で、最も適切な「戻し方」を答えてください。

- (a) Claude Code が作業中だが、明らかに頼んだのと違うファイルを編集し始めた
- (b) コミットしていない状態で、Claude が `Edit` ツールで書き換えた3つのファイルを、2つ前の指示を送った時点の状態に戻したい
- (c) 昨日コミットして push 済みの変更に不具合があったので、その変更を取り消したい

### 問題7: 同じバグで3回つまずいた（12-3）

`sortBooks` の評価順のバグを直そうと、同じ会話で3回修正を頼みましたが、まだ直りません。このあと別の機能（書籍の削除）にも取りかかる予定です。どうするのがよいですか？

### 問題8: AI のコードの問題を見つける（9-8）

Claude Code が次のコードを提案しました。問題点と、それを機械的に見つける方法、正しい書き方を答えてください。

```ts
// サーバー用の Supabase クライアントを作る関数を読み込む
import { createClient } from "@/lib/supabase/server";

// 書籍をタイトル順に取得する
export async function getBooks() {
  const supabase = await createClient();
  const { data } = await supabase.from("books").select("*").sortBy("title");
  return data;
}
```

### 解答と解説

#### 問題1の解答

```text
src/app/page.tsx だけを変更してください。

目的:
登録した本が何冊あるかを、一覧を見た瞬間に分かるようにしたい。

変更内容:
- 書籍一覧の上に「全 N 冊」（N は取得した書籍の件数）と表示する
- 0 冊のときは表示しない（今ある「書籍が登録されていません」の案内だけにする）

守ること:
- BookList などほかのファイルは変更しない

確かめ方:
- npm run lint と npx tsc --noEmit を実行し、結果を報告してください
- 私は npm run dev で、本が 0 冊・3 冊のときの表示を確認します
```

**解説:** 「いい感じに」を、①目的（冊数がすぐ分かる）、②場所と範囲（`src/app/page.tsx` だけ）、③完成条件（「全 N 冊」と表示、0冊のときは出さない）、④確かめ方（lint と型チェック、あなたのブラウザ確認）に分解しました。特に「0冊のとき」のような**境目のケース**を書いておくと、AI の推測に任せる部分が減ります。文面は人によって違ってかまいません。4つの要素がそろっていれば正解です。

#### 問題2の解答

| コマンド | 結果 | 理由 |
|---|---|---|
| (a) `npm run lint` | 確認なしで実行 | deny・ask には一致せず、allow の `Bash(npm run *)` に一致 |
| (b) `npm run deploy` | ブロック | allow にも一致するが、先に調べる deny の `Bash(npm run deploy)` に一致 |
| (c) `git commit -m "…"` | 確認が出る | allow にも書いてあるが、先に調べる ask に一致 |
| (d) `npm install zod` | 確認が出る | どのルールにも一致せず、Manual モードなので確認 |

**解説:** ルールは **deny → ask → allow** の順に調べられ、最初に一致したもので決まります（図11-9）。(b) と (c) のように、allow に同じものが入っていても、deny や ask が先に勝ちます。(d) のように「どれにも一致しない」ときの動きはモードしだいで、Manual なら確認が出ます。

#### 問題3の解答

1. **`sortBooks.test.ts` の差分を読む**: `git diff src/lib/sortBooks.test.ts` で、テストの期待値が書き換えられていないかを確認します（つまずきポイント⑪）。
2. **頼んでいない `server.ts` を戻す**: `git diff src/lib/supabase/server.ts` で中身を確認し、不要なら `git restore src/lib/supabase/server.ts` で元に戻します。
3. **頼んでいない `SortSelect.tsx` を消す**: 新しいファイル（`??`）は `git restore` では消えないので、今回使わないならファイルを削除します（次の作業で使うなら、別のコミットに回します）。
4. **自分でテストを実行**: `npx vitest run` を自分で実行して、報告どおり通ることを確かめます。
5. **頼んだ2ファイルだけコミット**: `git add src/lib/sortBooks.ts src/lib/sortBooks.test.ts` の後に `git commit` します。

**解説:** 「テストも通りました」という報告は、差分を読むまでは「自己申告」にすぎません。`git status` で頼んでいないファイル（`server.ts`・`SortSelect.tsx`）を見つけたら、中身を確かめてから戻すか、別の作業として切り分けます。テストファイルが変更されている場合は、期待値が都合よく変えられていないかを必ず確認しましょう。

#### 問題4の解答

- (a) **CLAUDE.md**（どの作業でも守ってほしい決まり）
- (b) **Hooks**（`PostToolUse` で Edit/Write の後に必ず実行。2-5）
- (c) **Permissions**（`deny` に `Read(.env.local)` を入れる。2-2）
- (d) **Skills**（`/check` のようなスキルを作る。7-6-1）
- (e) **Rules**（`.claude/rules/` に `paths` 付きで置く。`src/app/api/CLAUDE.md` でも可）

**解説:** (b) と (c) は「AI の判断に関係なく必ず効いてほしい」ことなので、お願いである CLAUDE.md ではなく、仕組みで強制する Hooks・Permissions を使います（1-4 の図11-8）。(a) と (e) の違いは、全体に効かせるか、特定の場所だけに効かせるかです。

#### 問題5の解答

`No, keep planning` を選び、次のように伝えます。

```text
計画から「3. src/app/layout.tsx の変更」を外してください。
今回は sortBooks.ts と sortBooks.test.ts の2ファイルだけを作ります。
ヘッダーのデザインは変更しないでください。
```

**解説:** 承認（`Yes, …`）を選ぶと、その計画のまま実装が始まってしまいます。計画の段階で範囲外の変更を見つけたら、`No, keep planning` で計画を直させるのがいちばん手間がかかりません。計画が直ったことを確認してから、`Yes, manually approve edits` で承認しましょう。

#### 問題6の解答

- (a) **`Esc` で止める**。そのうえで「〇〇は編集しないでください」と指示し直します。
- (b) **`Esc` を2回（または `/rewind`）** で巻き戻しメニューを開き、2つ前の指示の時点を選んで「コードを戻す（Restore code）」を選びます。
- (c) **`git revert コミットID`** で、その変更を打ち消すコミットを新しく作ります。

**解説:** (b) のチェックポイントは、Claude が `Edit`・`Write` で行った変更なら戻せますが、`rm` などの Bash コマンドで行った変更は戻せません。(c) のように push 済みのコミットは、履歴を書き換える `git reset --hard` ではなく、打ち消しのコミットを追加する `git revert` を使うのが安全です（チームの履歴を壊しません）。

#### 問題7の解答

`/clear` で会話をリセットし、ここまでに分かったこと（どの入力で、何が期待と違うか、どのテストが落ちるか）を書いた**新しい良い指示**で、もう一度頼みます。削除機能に移るときも、その前に `/clear` します。

**解説:** 同じ会話で失敗した修正を重ねると、コンテキスト（作業机）が失敗した案で埋まり、Claude がかえって混乱します。公式ドキュメントでも、2回以上修正してもうまくいかないときは `/clear` してやり直すことが勧められています。関係のない次の作業に移るときも `/clear` が基本です。CLAUDE.md は `/clear` の後も読み込まれるので、決まりごとが消える心配はありません。

#### 問題8の解答

**問題点:** `sortBy` というメソッドは supabase-js に存在しません（AI のハルシネーション）。

**見つけ方:** `npx tsc --noEmit` で型チェックをすると、次のエラーが出ます（supabase-js 2.117.3 / TypeScript 7.0.2 で確認）。

```text
src/lib/getBooks.ts(5,61): error TS2339: Property 'sortBy' does not exist on type 'PostgrestFilterBuilder<any, any, any, any[], "books", unknown, "GET", false>'.
```

**正しい書き方:**

```ts
// サーバー用の Supabase クライアントを作る関数を読み込む
import { createClient } from "@/lib/supabase/server";

// 書籍をタイトル順に取得する
export async function getBooks() {
  const supabase = await createClient();
  // select で取得し、order で並べ替える
  const { data } = await supabase
    .from("books")
    .select("*")
    .order("title", { ascending: true });
  return data;
}
```

**解説:** 見た目が自然なので読むだけでは見落としがちですが、型チェックなら確実に見つかります（9-8 の関所2）。あわせて、元のコードは `error` を受け取っていないので、取得に失敗しても気づけません。第7章のように `error` も受け取って確認するのが安全です。
