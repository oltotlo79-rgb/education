# 第5章: Supabase セットアップとデータベース設計

> この章では、**Supabase**（スーパベース：BaaSと呼ばれる「バックエンドまるごと提供サービス」）というサービスを使って、アプリのデータを保存する**データベース**（Database：データを永続的に保管する仕組み。アプリを閉じてもデータが消えない箱のようなもの）を準備します。

### この章で学ぶこと

- **データベースとは何か** — プログラムのデータを永続的に保存する「倉庫」
- **テーブル設計** — データをどのような「表」の形で保存するか考える作業
- **SQL の基本** — データベースに命令を出すための言語（SELECT, INSERT, UPDATE, DELETE）
- **RLS（Row Level Security：行レベルセキュリティ。「どの行に誰がアクセスできるか」を行ごとに制御する仕組み）** — 「誰がどのデータにアクセスできるか」を制御するセキュリティ機能
- **Supabase クライアント** — JavaScriptのコードからSupabaseに接続する方法

> **データベースはなぜ必要？** 変数や配列にデータを保持しても、ブラウザを閉じれば消えてしまいます。データベースはハードディスクにデータを保存するので、アプリを再起動してもデータが残ります。SNSの投稿、ECサイトの商品情報、そしてこのアプリの書籍情報も、すべてデータベースに保存されています。

> **補足: 用語の読み方**
> - **BaaS**（バース／ビーエーエーエス：Backend as a Service の略。サーバーやDBを自前で構築せずに使えるクラウドサービス）
> - **PostgreSQL**（ポストグレスキューエル：オープンソースのリレーショナルデータベース。Supabaseの中核）
> - **JWT**（ジョット／ジェイダブリュティー：JSON Web Token の略。ログイン状態を表すデジタル証明書のような文字列）
> - **Auth**（オース：Authentication = 認証。ログイン処理）
> - **Storage**（ストレージ：画像やファイルを保管する場所）
> - **リアルタイムサブスクリプション**（Realtime Subscription：DBの変更を即座に画面へ反映する仕組み。WebSocketで通知を受け取る）

## 目次

0. [前提知識: データベースとSQLの超基礎](#0-前提知識-データベースとsqlの超基礎)
1. [Supabase とは](#1-supabase-とは)
2. [アカウント作成とプロジェクトセットアップ](#2-アカウント作成とプロジェクトセットアップ)
3. [データベース設計の基礎](#3-データベース設計の基礎)
4. [書籍管理アプリのデータベース設計](#4-書籍管理アプリのデータベース設計)
5. [Row Level Security (RLS)](#5-row-level-security-rls)
6. [Supabase Client のセットアップ](#6-supabase-client-のセットアップ)
7. [Supabase の基本操作（CRUD）](#7-supabase-の基本操作crud)
8. [テストデータの投入](#8-テストデータの投入)
9. [トラブルシューティング](#9-トラブルシューティング)
10. [発展: アプリでは使っていない重要なSupabase機能](#発展-アプリでは使っていない重要なsupabase機能)

---

## 0. 前提知識: データベースとSQLの超基礎

Supabase の中身は **PostgreSQL（ポストグレスキューエル：30年以上の歴史を持つオープンソースのリレーショナルデータベース管理システム）** という有名なデータベースです。本格的に使い始める前に、データベース・テーブル・SQL のいろはを押さえておきましょう。

### 0.1 データベースとは何か

「データを永続的に・大量に・整理して保管しておく仕組み」がデータベース（DB）です。Excelで言えば「複数のシートを持つ巨大な台帳」のイメージです。本書で使う **PostgreSQL** は、最も広く使われている**リレーショナルデータベース（RDB：Relational Database。表の形でデータを管理し、表同士を関連付けられるDB）** の一つです。

### 0.2 テーブル・カラム・レコード

RDB では、データを **テーブル（表）** に保存します。

```
┌───── booksテーブル ─────────────────────────────────┐
│ id │ title              │ author       │ status   │  ← カラム（列）
├────┼────────────────────┼──────────────┼──────────┤
│  1 │ リーダブルコード     │ Boswell        │ done     │  ← レコード（行）
│  2 │ プロを目指す人〜     │ 鈴木僚太       │ reading  │
│  3 │ 達人プログラマー    │ Thomas         │ unread   │
└─────────────────────────────────────────────────────┘
```

| 用語 | 意味 | Excelで言うと |
|------|------|---------------|
| テーブル | データの表そのもの | シート1枚 |
| カラム（列） | 縦の項目（id, title, author...） | A列、B列、C列... |
| レコード（行） | 横の1件分のデータ | 1行目、2行目... |
| 主キー（PK） | レコードを一意に区別する値（普通は `id`） | 通し番号 |

### 0.3 SQL の超ミニマム入門

データベースを操作する言語が **SQL（エスキューエル：Structured Query Language の略。データベース操作の世界標準言語）** です。よく使う4つだけ先に覚えましょう。

#### SELECT — データを取り出す

> **▼ このコードがやること（先に日本語で）:** テーブルに保存済みのデータを**取り出して表示する**命令です。SQL では「SELECT（取り出す列）FROM（どのテーブルから）」の順で書きます。Excel で言えば「この表のこの列だけ見せて」とお願いするイメージです。

```sql
-- SELECT : 「テーブルからデータを取り出す」SQL文の開始キーワード
-- title, author : 取り出したいカラム名をカンマ区切りで指定（必要な列だけ取れる）
-- FROM books   : どのテーブルから取るかを指定（ここでは books テーブル）
-- ;            : SQL文の終わりを表す記号（必ず必要）
SELECT title, author FROM books;
```

**▼ 実行結果:**
```
title             | author
------------------|----------
リーダブルコード   | Boswell
プロを目指す人〜   | 鈴木僚太
達人プログラマー   | Thomas
```

条件付きで取り出すこともできます。

> **▼ このコードがやること（先に日本語で）:** 全部ではなく「**条件に合う行だけ**」を取り出します。`WHERE` のうしろに「どんな行が欲しいか」の条件を書くのがポイントです。ここでは「status が reading の本だけ見せて」とお願いしています。

```sql
-- SELECT *           : * は「全カラム」を意味するワイルドカード
-- FROM books         : booksテーブルから取り出す
-- WHERE status='reading' : 「statusカラムが'reading'と等しい行」だけに絞り込む条件
-- 文字列リテラルはシングルクォート ' で囲む（SQLのルール）
SELECT * FROM books WHERE status = 'reading';
```

**▼ 実行結果（statusが'reading'の行だけ）:**
```
id | title              | author       | status
---|--------------------|--------------|--------
 2 | プロを目指す人〜   | 鈴木僚太     | reading
```

#### INSERT — データを新しく追加する

> **▼ このコードがやること（先に日本語で）:** テーブルに**新しい行（レコード）を1件追加する**命令です。「どの列に」「どんな値を」入れるかを、`(列名...)` と `VALUES (値...)` で**同じ順番**にそろえて書くのがコツです。表に新しい1行を書き足すイメージです。

```sql
-- INSERT INTO books     : booksテーブルに新しい行を追加する宣言
-- (title, author, status): どのカラムに値を入れるか、カラム名を列挙
-- VALUES (..., ..., ...): 上で並べたカラムと同じ順番で値を指定
-- 'SQL入門'             : title カラムに入れる文字列リテラル
-- 'ミック'              : author カラムに入れる文字列
-- 'unread'              : status カラムに入れる文字列
INSERT INTO books (title, author, status) VALUES ('SQL入門', 'ミック', 'unread');
```

**▼ 実行結果:** `INSERT 0 1`（1件追加された）

実行後にテーブルを SELECT すると、新しい行が増えています。

#### UPDATE — データを書き換える

> **▼ このコードがやること（先に日本語で）:** すでにある行の値を**書き換える（更新する）**命令です。`SET` で「どの列をどの値に変えるか」を、`WHERE` で「どの行を対象にするか」を指定します。**`WHERE` を忘れると全行が書き換わってしまう**ので、必ず対象を絞り込むのが最重要ポイントです。

```sql
-- UPDATE books     : books テーブルの既存行を書き換える宣言
-- SET status='done': どのカラムをどの値に変えるかを指定（=は代入）
-- WHERE id = 2     : 「id が 2 の行だけ」更新対象にする絞り込み条件
--                    WHERE を忘れると全行が書き換わるので超危険
UPDATE books SET status = 'done' WHERE id = 2;
```

**▼ 実行結果:** `UPDATE 1`（1件更新された）。id=2 のレコードの status が `'reading'` から `'done'` に変わります。

> **`WHERE` を必ず付ける!**: `WHERE` を忘れると**全レコードが書き換わってしまいます**。常に「どの行を対象にするか」を明示しましょう。

#### DELETE — データを消す

> **▼ このコードがやること（先に日本語で）:** テーブルから**行を削除する**命令です。`WHERE` で「どの行を消すか」を必ず指定します。**`WHERE` を忘れると全件削除（しかも復元不可）**になるため、削除対象の絞り込みが命綱だと覚えてください。

```sql
-- DELETE FROM books : books テーブルから行を削除する宣言
-- WHERE id = 3      : 「id が 3 の行だけ」削除する条件
--                     WHEREを忘れると全件削除で復元不可なので絶対に注意
DELETE FROM books WHERE id = 3;
```

**▼ 実行結果:** `DELETE 1`（1件削除された）。

> **これも `WHERE` 必須!**: 忘れると全件削除です。バックアップ無しで実行すると地獄を見ます。

### 0.4 CRUDという言葉

上の4つの操作を頭文字でまとめて **CRUD（クラッド：Create / Read / Update / Delete の頭文字）** と呼びます。

| C | Create | INSERT | データを作る |
| R | Read   | SELECT | データを読む |
| U | Update | UPDATE | データを更新する |
| D | Delete | DELETE | データを削除する |

ほぼすべてのアプリは、CRUD のどれか（または組み合わせ）で動いています。本書の書籍管理アプリも、CRUD の練習が目的です。

### 0.5 本書での書き方: SQL 直接書かない

本書では、これらの SQL を**直接書く機会は少なめ**です。代わりに **Supabase のJavaScriptクライアント**を使い、TypeScriptのコードで操作します。

> **▼ このコードがやること（先に日本語で）:** さっきの SQL（`SELECT * FROM books`）を、**TypeScript のコードで同じことをする**書き方に置き換えた例です。`supabase.from("books").select("*")` が「booksテーブルから全部取り出す」にあたります。SQL を直接書かなくても、メソッドをつなげるだけでDB操作できる、という感覚をつかむのが目的です。

```typescript
// SQLの「SELECT * FROM books」と同じ意味のコード
// await       : このAPI呼び出しは非同期（時間がかかる）ので結果が返るまで待つ
// supabase    : 別ファイルで作成した Supabase クライアントオブジェクト
// .from("books"): 操作対象のテーブル名を指定（戻り値はクエリビルダ）
// .select("*")  : 取得するカラムを指定。"*" は全カラム
// 分割代入 { data } : 戻り値オブジェクトから data プロパティだけを取り出す
const { data } = await supabase.from("books").select("*");
// console.log : ブラウザ開発者ツールのConsoleタブに値を出力する関数
console.log(data);
// ▼ data の中身（実行結果のイメージ）
// [
//   { id: 1, title: "リーダブルコード", author: "Boswell", status: "done" },
//   { id: 2, title: "プロを目指す人〜", author: "鈴木僚太", status: "reading" },
//   ...
// ]
```

JavaScript のコードと SQL の対応関係は次のとおり。

| 操作 | Supabase クライアント | SQL |
|------|----------------------|-----|
| 読む | `.from("books").select("*")` | `SELECT * FROM books` |
| 追加 | `.from("books").insert({ title: "..." })` | `INSERT INTO books (...)` |
| 更新 | `.from("books").update({ status: "done" }).eq("id", 2)` | `UPDATE books SET status='done' WHERE id=2` |
| 削除 | `.from("books").delete().eq("id", 3)` | `DELETE FROM books WHERE id=3` |

> **要点:** 「SQLは知らなくても書ける」けど「SQLを知っていると見通しがよくなる」。Supabaseには**SQLエディタ画面**があるので、迷ったらそこで生のSQLを書いて確認できます。

---

## 1. Supabase とは

### 1.1 BaaS (Backend as a Service) の概念

Web アプリケーションを作るとき、通常は「フロントエンド（画面）」と「バックエンド（サーバー・データベース）」の両方を構築する必要があります。

**従来の開発フロー:**

1. データベースサーバーを用意する（PostgreSQL、MySQL など）
2. API サーバーを構築する（Express、Django、Rails など）
3. 認証の仕組みを実装する
4. ファイルストレージを用意する
5. これらすべてをデプロイ・運用する

これは初心者にとって非常に大きなハードルです。学ぶべき技術が多すぎて、アプリの本質的な部分に集中できません。

**BaaS（Backend as a Service：バックエンドをサービスとして利用する形態。Firebase や Supabase など）** は、このバックエンド部分をまるごとクラウドサービスとして提供してくれるものです。つまり、データベース・認証・ストレージ・API といったバックエンドの機能を、自分でサーバーを構築することなく利用できます。

```
従来の開発:
  フロントエンド → 自作 API サーバー → 自作データベース → 自作認証 → 自作ストレージ
  （すべて自分で構築・運用）

BaaS を使った開発:
  フロントエンド → BaaS（Supabase）
  （バックエンドはすべて Supabase が提供）
```

### 1.2 Supabase の全体アーキテクチャ

Supabase がどのような構成になっているか、全体像を見てみましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- Client Layer -->
  <div style="background: #eff6ff; border: 2px solid #3b82f6; border-radius: 12px; padding: 16px; margin-bottom: 12px; text-align: center;">
    <div style="font-size: 11px; color: #64748b; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px;">Client (Browser / App)</div>
    <div style="background: white; border: 1px solid #bfdbfe; border-radius: 8px; padding: 12px 20px; display: inline-block;">
      <div style="font-size: 20px;">🖥️</div>
      <div style="font-weight: 700; color: #1e40af; font-size: 13px;">Frontend</div>
      <div style="font-size: 11px; color: #64748b;">React / Next.js</div>
    </div>
  </div>
  <!-- Arrow -->
  <div style="text-align: center; color: #3b82f6; font-size: 20px; margin: 4px 0;">⬇️ HTTPS ⬆️</div>
  <!-- Supabase Platform -->
  <div style="background: #faf5ff; border: 2px solid #8b5cf6; border-radius: 12px; padding: 16px;">
    <div style="font-size: 11px; color: #64748b; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px; text-align: center;">Supabase Platform</div>
    <!-- API Gateway -->
    <div style="background: #fff7ed; border: 1px solid #f59e0b; border-radius: 8px; padding: 10px 16px; text-align: center; margin-bottom: 12px;">
      <span style="font-size: 16px;">🌐</span>
      <span style="font-weight: 700; color: #92400e; font-size: 13px; margin-left: 6px;">Kong (API Gateway)</span>
    </div>
    <!-- Arrow -->
    <div style="text-align: center; color: #8b5cf6; font-size: 14px; margin: 4px 0;">↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓</div>
    <!-- Services Row -->
    <div style="display: flex; gap: 8px; flex-wrap: wrap; justify-content: center; margin-bottom: 12px;">
      <div style="background: white; border: 1px solid #d8b4fe; border-radius: 8px; padding: 10px 12px; text-align: center; flex: 1; min-width: 100px;">
        <div style="font-size: 16px;">🔐</div>
        <div style="font-weight: 700; color: #6d28d9; font-size: 12px;">GoTrue</div>
        <div style="font-size: 10px; color: #94a3b8;">Auth</div>
      </div>
      <div style="background: white; border: 1px solid #d8b4fe; border-radius: 8px; padding: 10px 12px; text-align: center; flex: 1; min-width: 100px;">
        <div style="font-size: 16px;">🔌</div>
        <div style="font-weight: 700; color: #6d28d9; font-size: 12px;">PostgREST</div>
        <div style="font-size: 10px; color: #94a3b8;">REST API</div>
      </div>
      <div style="background: white; border: 1px solid #d8b4fe; border-radius: 8px; padding: 10px 12px; text-align: center; flex: 1; min-width: 100px;">
        <div style="font-size: 16px;">⚡</div>
        <div style="font-weight: 700; color: #6d28d9; font-size: 12px;">Realtime</div>
        <div style="font-size: 10px; color: #94a3b8;">WebSocket</div>
      </div>
      <div style="background: white; border: 1px solid #d8b4fe; border-radius: 8px; padding: 10px 12px; text-align: center; flex: 1; min-width: 100px;">
        <div style="font-size: 16px;">📦</div>
        <div style="font-weight: 700; color: #6d28d9; font-size: 12px;">Storage API</div>
        <div style="font-size: 10px; color: #94a3b8;">File</div>
      </div>
      <div style="background: white; border: 1px solid #d8b4fe; border-radius: 8px; padding: 10px 12px; text-align: center; flex: 1; min-width: 100px;">
        <div style="font-size: 16px;">🦕</div>
        <div style="font-weight: 700; color: #6d28d9; font-size: 12px;">Deno Runtime</div>
        <div style="font-size: 10px; color: #94a3b8;">Edge Functions</div>
      </div>
    </div>
    <!-- Arrow -->
    <div style="text-align: center; color: #3ecf8e; font-size: 14px; margin: 4px 0;">↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓</div>
    <!-- Database -->
    <div style="background: #ecfdf5; border: 2px solid #3ecf8e; border-radius: 8px; padding: 12px 16px; text-align: center;">
      <span style="font-size: 18px;">🐘</span>
      <span style="font-weight: 700; color: #065f46; font-size: 14px; margin-left: 6px;">PostgreSQL (Main Database)</span>
    </div>
  </div>
</div>

**ポイント:** Supabase はオープンソースのツールを組み合わせて構築されています。PostgreSQL を中核として、PostgREST（自動 API 生成）、GoTrue（認証）、Realtime（リアルタイム通信）などが連携しています。

> **登場する各サービスの読み方と役割:**
> - **Kong**（コング：API Gateway。すべてのリクエストを最初に受け取る玄関口）
> - **GoTrue**（ゴートゥルー：認証サーバー。ログイン・サインアップ・JWT発行を担当）
> - **PostgREST**（ポストグレストレスト：DBのテーブル定義から自動的にRESTful APIを生成するソフトウェア）
> - **Realtime**（リアルタイム：DBの変更を WebSocket でブラウザに即時配信するサーバー）
> - **Storage**（ストレージ：画像やPDFなどファイルを保管するS3互換のサービス）
> - **Edge Functions**（エッジファンクション：世界各地のサーバーで動く小さな関数。Deno で実行される）

### 1.3 Firebase との比較

BaaS の中で最も有名なのは Google の **Firebase** です。Supabase は「Firebase のオープンソース代替」を目指して作られました。両者を比較してみましょう。

| 項目 | Supabase | Firebase |
|------|----------|----------|
| **データベース** | PostgreSQL（リレーショナル DB） | Firestore（NoSQL ドキュメント DB） |
| **クエリ言語** | SQL（標準的で汎用性が高い） | 独自クエリ API |
| **リアルタイム** | PostgreSQL の変更検知 | ネイティブリアルタイム |
| **認証** | GoTrue（メール、OAuth 等） | Firebase Auth（メール、OAuth 等） |
| **ストレージ** | S3 互換ストレージ | Cloud Storage |
| **サーバーレス関数** | Edge Functions（Deno） | Cloud Functions（Node.js） |
| **料金** | 無料枠あり、従量課金 | 無料枠あり、従量課金 |
| **オープンソース** | はい（セルフホスト可能） | いいえ |
| **ベンダーロックイン** | 低い（標準 SQL で移行しやすい） | 高い（独自仕様が多い） |
| **学習の汎用性** | SQL は他の仕事でも使える | Firestore の知識は限定的 |
| **複雑なクエリ** | JOIN、サブクエリなど強力 | 限定的（非正規化が必要） |
| **型安全性** | CLI で TypeScript 型を自動生成可能 | 手動で型定義が必要 |

### 1.4 PostgreSQL ベースであることの利点

Supabase が PostgreSQL を採用していることには大きなメリットがあります。

**1. SQL は業界標準のスキル**

SQL（Structured Query Language）は、1970年代から使われ続けている データベース操作の標準言語です。Web 開発だけでなく、データ分析、機械学習、業務システムなど、あらゆる分野で使われています。Supabase で SQL を学ぶことは、他のどんなプロジェクトでも役立つスキルを身につけることを意味します。

**2. リレーショナルデータベースの信頼性**

PostgreSQL は 30年以上の歴史を持つ、世界で最も信頼されているオープンソースデータベースの一つです。データの整合性を厳密に保証する仕組み（トランザクション、制約など）が備わっています。

**3. 豊富な機能**

- **JSON サポート**: NoSQL のような柔軟なデータも格納可能
- **全文検索**: テキスト検索機能を内蔵
- **地理空間データ**: PostGIS 拡張で位置情報も扱える
- **拡張機能**: 数百の拡張機能でほぼあらゆる用途に対応

**4. 移行のしやすさ**

標準 SQL を使っているため、将来 Supabase 以外のサービス（AWS RDS、Google Cloud SQL、自前サーバーなど）に移行する場合でも、データベースの知識とコードの大部分をそのまま活かせます。

### 1.5 Supabase が提供する機能一覧

| 機能 | 説明 | 本チュートリアルでの使用 |
|------|------|--------------------------|
| **Database** | PostgreSQL データベース | 書籍データの保存 |
| **Auth** | ユーザー認証（メール、OAuth、マジックリンク等） | 第7章で使用予定 |
| **Storage** | ファイルアップロード・管理 | 書籍カバー画像の保存（応用編） |
| **Realtime** | データ変更のリアルタイム配信 | 応用編で使用予定 |
| **Edge Functions** | サーバーレス関数（Deno ランタイム） | 応用編で使用予定 |
| **Auto-generated API** | テーブルから REST / GraphQL API を自動生成 | 本章でメインで使用 |
| **Dashboard** | Web ベースの管理画面 | テーブル作成・データ確認 |
| **CLI** | コマンドラインツール | 型の自動生成 |

---

## 2. アカウント作成とプロジェクトセットアップ

### 2.1 Supabase アカウントの作成

**ステップ 1: Supabase 公式サイトにアクセス**

ブラウザで以下の URL にアクセスします。

```
https://supabase.com
```

トップページが表示されたら、右上の「Start your project」ボタンをクリックします。

**ステップ 2: GitHub アカウントでサインアップ**

Supabase は GitHub アカウントでのサインアップを推奨しています（GitHub 連携のみ対応）。

1. 「Sign in with GitHub」ボタンをクリック
2. GitHub のログイン画面が表示されるので、自分の GitHub アカウントでログイン
3. Supabase にアクセスを許可するかの確認画面が出るので「Authorize supabase」をクリック
4. Supabase のダッシュボードにリダイレクトされれば成功

> **GitHub アカウントを持っていない場合:**
> まず https://github.com でアカウントを作成してください。GitHub は開発者にとって必須のサービスなので、この機会に作成しておきましょう。

**ステップ 3: ダッシュボードの確認**

ログイン後、Supabase のダッシュボード画面が表示されます。初回ログイン時はプロジェクトが一つもない状態です。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08);">
  <div style="background: #1c1c1c; padding: 14px 24px; display: flex; justify-content: space-between; align-items: center;">
    <span style="color: #3ecf8e; font-weight: 700; font-size: 16px;">⚡ Supabase</span>
    <span style="color: #888; font-size: 13px;">Dashboard</span>
  </div>
  <div style="background: #f8fafc; padding: 48px 24px; text-align: center;">
    <div style="font-size: 48px; margin-bottom: 16px;">🗄️</div>
    <div style="font-size: 16px; color: #64748b; margin-bottom: 20px;">No projects yet</div>
    <div style="display: inline-block; background: #3ecf8e; color: white; padding: 10px 24px; border-radius: 8px; font-weight: 600; font-size: 14px;">+ New Project</div>
  </div>
</div>

### 2.2 新規プロジェクトの作成

**ステップ 1: 「New Project」ボタンをクリック**

ダッシュボードの「New Project」ボタンをクリックします。

**ステップ 2: Organization（組織）の選択**

初回の場合は、自動的に個人用の Organization が作られています。そのまま選択してください。

**ステップ 3: プロジェクト情報の入力**

以下の情報を入力します。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08);">
  <div style="background: #1c1c1c; padding: 16px 24px;">
    <span style="color: #f1f5f9; font-weight: 700; font-size: 18px;">Create a new project</span>
  </div>
  <div style="background: #181818; padding: 28px 32px;">
    <div style="margin-bottom: 20px;">
      <div style="color: #a1a1aa; font-size: 13px; margin-bottom: 6px; font-weight: 500;">Name</div>
      <div style="background: #272727; border: 1px solid #3f3f46; border-radius: 6px; padding: 10px 14px; color: #e4e4e7; font-size: 14px;">book-manager</div>
    </div>
    <div style="margin-bottom: 20px;">
      <div style="color: #a1a1aa; font-size: 13px; margin-bottom: 6px; font-weight: 500;">Database Password</div>
      <div style="background: #272727; border: 1px solid #3f3f46; border-radius: 6px; padding: 10px 14px; color: #e4e4e7; font-size: 14px; letter-spacing: 2px;">●●●●●●●●●●●●●●●●</div>
      <div style="margin-top: 8px;">
        <span style="color: #3ecf8e; font-size: 13px; cursor: pointer; font-weight: 500;">Generate a password</span>
      </div>
    </div>
    <div style="margin-bottom: 20px;">
      <div style="color: #a1a1aa; font-size: 13px; margin-bottom: 6px; font-weight: 500;">Region</div>
      <div style="background: #272727; border: 1px solid #3f3f46; border-radius: 6px; padding: 10px 14px; color: #e4e4e7; font-size: 14px; display: flex; justify-content: space-between; align-items: center;">
        <span>Northeast Asia (Tokyo)</span>
        <span style="color: #71717a; font-size: 12px;">▼</span>
      </div>
    </div>
    <div style="margin-bottom: 28px;">
      <div style="color: #a1a1aa; font-size: 13px; margin-bottom: 6px; font-weight: 500;">Pricing Plan</div>
      <div style="background: #272727; border: 1px solid #3f3f46; border-radius: 6px; padding: 10px 14px; color: #e4e4e7; font-size: 14px; display: flex; justify-content: space-between; align-items: center;">
        <span>Free - $0/month</span>
        <span style="color: #71717a; font-size: 12px;">▼</span>
      </div>
    </div>
    <div style="text-align: center;">
      <div style="display: inline-block; background: #3ecf8e; color: #1c1c1c; padding: 10px 32px; border-radius: 8px; font-weight: 600; font-size: 14px;">Create new project</div>
    </div>
  </div>
</div>

各項目の詳細:

| 項目 | 入力値 | 説明 |
|------|--------|------|
| **Name** | `book-manager` | プロジェクト名。分かりやすい名前をつけましょう |
| **Database Password** | （自動生成推奨） | 「Generate a password」をクリックして安全なパスワードを生成。**必ずどこかにメモしておくこと** |
| **Region** | `Northeast Asia (Tokyo)` | データベースが配置される地域。日本在住なら Tokyo を選択 |
| **Pricing Plan** | `Free` | 無料プラン。学習には十分な機能が利用可能 |

**ステップ 4: プロジェクトの作成**

「Create new project」ボタンをクリックします。プロジェクトのセットアップには 1〜2 分程度かかります。画面に進捗バーが表示されるので、完了するまで待ちます。

**ステップ 5: プロジェクトダッシュボードの確認**

セットアップが完了すると、プロジェクトのダッシュボードが表示されます。

```
┌──────────────────────────────────────────────────────────┐
│  book-manager                                            │
│                                                          │
│  ┌─────────────────┐  ┌─────────────────┐               │
│  │ Project URL      │  │ API Key (anon)  │               │
│  │ https://xxxx     │  │ eyJhbGci...     │               │
│  │ .supabase.co     │  │                 │               │
│  └─────────────────┘  └─────────────────┘               │
│                                                          │
│  ── Getting Started ──                                   │
│  [Table Editor] [SQL Editor] [Auth] [Storage]            │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

> **重要: API キーを控えておく**
>
> ダッシュボードの「Settings」>「API」から以下の2つの値を確認・メモしてください。後ほどコードから Supabase に接続する際に必要です。
>
> - **Project URL**: `https://xxxxxxxx.supabase.co` の形式
> - **anon (public) key**: `eyJhbGciOiJIUzI1NiIs...` のような長い文字列（実体は **JWT**：JSON Web Token と呼ばれる署名付きの文字列）

> **ANON Key と Service Role Key の違い（重要）**
>
> Supabase のプロジェクトには2種類のAPIキーが発行されます。混同すると重大な事故になります。
>
> | キー名 | 用途 | RLSの効果 | どこに置く |
> |--------|------|-----------|-----------|
> | **anon key**（アノン：anonymous の略。匿名キー） | ブラウザから使う公開キー | RLS が有効。ポリシーに従う | フロントエンド、`.env.local` の `NEXT_PUBLIC_` プレフィックス付きでOK |
> | **service_role key**（サービスロール：管理者キー） | サーバー処理から使う特権キー | **RLSを完全にバイパス（無視）** | **絶対にフロントエンドに含めない**。サーバー側だけ |
>
> **service_role key が漏えいすると、全データの読み取り・書き換え・削除を誰でもできるようになります**。Gitにコミットしたり、ブラウザに渡したりするのは絶対に避けてください。

### 2.3 リージョンの選び方

リージョンとは、Supabase のサーバー（データベース）が物理的に配置されるデータセンターの場所です。

**選び方の基本原則:**

> ユーザー（自分やアプリの利用者）に最も近いリージョンを選ぶ

日本在住で、主に日本国内向けのアプリを作る場合は **Northeast Asia (Tokyo)** を選択するのがベストです。

**なぜリージョンが重要なのか:**

データベースとの通信には物理的な距離に応じた遅延（レイテンシ）が発生します。

| リージョン | 日本からのおおよその遅延 |
|-----------|------------------------|
| Tokyo（東京） | 5〜15ms |
| Singapore（シンガポール） | 60〜80ms |
| US West（米国西部） | 100〜150ms |
| US East（米国東部） | 150〜200ms |
| EU West（ヨーロッパ西部） | 200〜300ms |

> **注意:** リージョンはプロジェクト作成後に変更できません。間違えた場合は新しいプロジェクトを作り直す必要があります。

### 2.4 無料プランの制限

Supabase の無料プラン（Free Plan）には以下の制限があります。学習目的であればまったく問題ありません。

| 項目 | 無料プランの制限 |
|------|-----------------|
| **プロジェクト数** | 2つまで |
| **データベース容量** | 500 MB |
| **ストレージ容量** | 1 GB |
| **帯域幅** | 5 GB / 月 |
| **Edge Function 呼び出し** | 50万回 / 月 |
| **認証ユーザー数** | 無制限（MAU ベースでない） |
| **一時停止** | 1週間アクセスがないと自動停止（再起動可） |

> **自動停止について:**
> 無料プランでは、1週間以上ダッシュボードやAPIへのアクセスがないとプロジェクトが自動的に一時停止されます。停止されてもデータは消えません。ダッシュボードにアクセスして「Restore project」ボタンを押せば数分で復旧します。学習中は定期的にアクセスするようにしましょう。

---

## 3. データベース設計の基礎

### 3.1 リレーショナルデータベースとは

**リレーショナルデータベース（RDB）** とは、データを「テーブル（表）」の形式で管理するデータベースです。Excel のスプレッドシートをイメージすると分かりやすいでしょう。

**例: 書籍情報のテーブル**

| id | title | author | rating |
|----|-------|--------|--------|
| 1 | ノルウェイの森 | 村上春樹 | 5 |
| 2 | 人間失格 | 太宰治 | 4 |
| 3 | 1Q84 | 村上春樹 | 5 |

このテーブルの各部分には名前がついています:

- **テーブル（Table）**: データのまとまり全体（上の表そのもの）。「books テーブル」のように名前をつけます
- **行（Row / Record）**: テーブル内の1件のデータ。上の例では3行 = 3冊の書籍データ
- **列（Column / Field）**: データの各項目。上の例では id, title, author, rating の4列

### 3.2 リレーショナルデータベースの全体像

実際のアプリケーションでは、複数のテーブルが関連し合います。例えば「ユーザー」と「書籍」のように、テーブル間に関係（リレーション）があるのが特徴です。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- Relationship Legend -->
  <div style="background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 10px; padding: 12px 16px; margin-bottom: 16px; font-size: 12px; color: #64748b; text-align: center;">
    <strong style="color: #334155;">Relationships:</strong>&nbsp;&nbsp;
    USERS <span style="color: #3b82f6;">||--o{</span> BOOKS (owns) &nbsp;&nbsp;
    USERS <span style="color: #3b82f6;">||--o{</span> REVIEWS (writes) &nbsp;&nbsp;
    BOOKS <span style="color: #3b82f6;">||--o{</span> REVIEWS (has) &nbsp;&nbsp;
    BOOKS <span style="color: #3b82f6;">}o--||</span> CATEGORIES (belongs to)
  </div>
  <!-- Tables Grid -->
  <div style="display: flex; flex-wrap: wrap; gap: 14px; justify-content: center;">
    <!-- USERS Table -->
    <div style="border: 2px solid #3b82f6; border-radius: 10px; overflow: hidden; min-width: 240px; flex: 1; max-width: 320px;">
      <div style="background: #1e40af; color: white; padding: 10px 16px; font-weight: 700; font-size: 14px; text-align: center;">👤 USERS</div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0; background: #f8fafc;">
        <span style="color: #f59e0b; font-weight: 700;">🔑</span> id <span style="color: #94a3b8; float: right;">uuid (PK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        email <span style="color: #94a3b8; float: right;">text</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        name <span style="color: #94a3b8; float: right;">text</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px;">
        created_at <span style="color: #94a3b8; float: right;">timestamptz</span>
      </div>
    </div>
    <!-- BOOKS Table -->
    <div style="border: 2px solid #3ecf8e; border-radius: 10px; overflow: hidden; min-width: 240px; flex: 1; max-width: 320px;">
      <div style="background: #065f46; color: white; padding: 10px 16px; font-weight: 700; font-size: 14px; text-align: center;">📚 BOOKS</div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0; background: #f8fafc;">
        <span style="color: #f59e0b; font-weight: 700;">🔑</span> id <span style="color: #94a3b8; float: right;">uuid (PK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        <span style="color: #3b82f6; font-weight: 700;">🔗</span> user_id <span style="color: #94a3b8; float: right;">uuid (FK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        <span style="color: #3b82f6; font-weight: 700;">🔗</span> category_id <span style="color: #94a3b8; float: right;">uuid (FK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        title <span style="color: #94a3b8; float: right;">text</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        author <span style="color: #94a3b8; float: right;">text</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        rating <span style="color: #94a3b8; float: right;">integer</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px;">
        created_at <span style="color: #94a3b8; float: right;">timestamptz</span>
      </div>
    </div>
    <!-- CATEGORIES Table -->
    <div style="border: 2px solid #8b5cf6; border-radius: 10px; overflow: hidden; min-width: 240px; flex: 1; max-width: 320px;">
      <div style="background: #5b21b6; color: white; padding: 10px 16px; font-weight: 700; font-size: 14px; text-align: center;">🏷️ CATEGORIES</div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0; background: #f8fafc;">
        <span style="color: #f59e0b; font-weight: 700;">🔑</span> id <span style="color: #94a3b8; float: right;">uuid (PK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px;">
        name <span style="color: #94a3b8; float: right;">text</span>
      </div>
    </div>
    <!-- REVIEWS Table -->
    <div style="border: 2px solid #f59e0b; border-radius: 10px; overflow: hidden; min-width: 240px; flex: 1; max-width: 320px;">
      <div style="background: #92400e; color: white; padding: 10px 16px; font-weight: 700; font-size: 14px; text-align: center;">📝 REVIEWS</div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0; background: #f8fafc;">
        <span style="color: #f59e0b; font-weight: 700;">🔑</span> id <span style="color: #94a3b8; float: right;">uuid (PK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        <span style="color: #3b82f6; font-weight: 700;">🔗</span> user_id <span style="color: #94a3b8; float: right;">uuid (FK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        <span style="color: #3b82f6; font-weight: 700;">🔗</span> book_id <span style="color: #94a3b8; float: right;">uuid (FK)</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        content <span style="color: #94a3b8; float: right;">text</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
        score <span style="color: #94a3b8; float: right;">integer</span>
      </div>
      <div style="padding: 8px 16px; font-size: 12px;">
        created_at <span style="color: #94a3b8; float: right;">timestamptz</span>
      </div>
    </div>
  </div>
</div>

> **注意:** 上の図は一般的な書籍管理システムの例です。本チュートリアルでは、まず **books テーブル1つ** から始め、後の章で認証やリレーションを追加していきます。

### 3.3 主キー（Primary Key）

**主キー（PK: Primary Key）** とは、テーブル内の各行を一意に識別するための列です。

**なぜ主キーが必要か？**

例えば、同じ「村上春樹」の「ノルウェイの森」が2冊登録された場合、どちらを指しているか区別できなくなります。主キーがあれば、`id = 1` と `id = 2` のように一意に特定できます。

**主キーのルール:**

1. テーブル内でユニーク（重複しない）こと
2. NULL（空）にはできないこと
3. 基本的に変更しないこと

**UUID vs 連番（SERIAL）:**

| 方式 | 例 | メリット | デメリット |
|------|-----|---------|-----------|
| **UUID**（ユーユーアイディー：Universally Unique Identifier。世界中で重複しない128ビットの識別子） | `550e8400-e29b-41d4-a716-446655440000` | 衝突の可能性がほぼゼロ。分散システムに適する | 長い。人間が読みにくい |
| **SERIAL**（シリアル：自動採番される連番） | `1`, `2`, `3` ... | 短い。人間が読みやすい | 連番が推測可能。分散環境で衝突しうる |

Supabase では **UUID** がデフォルトで推奨されています。本チュートリアルでも UUID を使用します。

### 3.4 外部キー（Foreign Key）

**外部キー（FK: Foreign Key）** とは、別のテーブルの主キーを参照する列です。テーブル間の関連付けに使います。

```
books テーブル              users テーブル
┌──────────┬───────────┐    ┌──────────┬──────────┐
│ id (PK)  │ user_id(FK)│   │ id (PK)  │ name     │
├──────────┼───────────┤    ├──────────┼──────────┤
│ uuid-001 │ user-AAA  │───→│ user-AAA │ 田中太郎 │
│ uuid-002 │ user-AAA  │───→│          │          │
│ uuid-003 │ user-BBB  │───→│ user-BBB │ 鈴木花子 │
└──────────┴───────────┘    └──────────┴──────────┘
```

上の例では、books テーブルの `user_id` が users テーブルの `id` を参照しています。これにより「どのユーザーがどの本を登録したか」が分かります。

> **本チュートリアルでは:** まだ認証機能を実装していないため、外部キーは後の章で追加します。今回は books テーブル単体で始めます。

### 3.5 データ型

PostgreSQL で使う主なデータ型を紹介します。

| データ型 | 説明 | 例 |
|---------|------|-----|
| `uuid` | 一意識別子（128ビット） | `550e8400-e29b-41d4-a716-446655440000` |
| `text` | 可変長テキスト（長さ制限なし） | `'ノルウェイの森'` |
| `varchar(n)` | 可変長テキスト（最大 n 文字） | `'abc'`（最大n文字） |
| `integer` | 整数（-2147483648 〜 2147483647） | `42` |
| `bigint` | 大きい整数 | `9223372036854775807` |
| `boolean`（bool） | 真偽値 | `true` / `false` |
| `date` | 日付 | `'2024-01-15'` |
| `timestamptz` | タイムスタンプ（タイムゾーン付き） | `'2024-01-15T10:30:00+09:00'` |
| `jsonb` | JSON データ（バイナリ形式） | `'{"key": "value"}'` |
| `numeric` | 精密な小数 | `3.14` |

> **`text` vs `varchar(n)` について:**
> PostgreSQL では `text` と `varchar` の性能差はほとんどありません。長さの制限が本当に必要な場合以外は `text` を使うのがシンプルです。Supabase の公式ドキュメントでも `text` が推奨されています。

> **`timestamptz` と `timestamp` の違い:**
> `timestamptz` は "timestamp with time zone" の略で、タイムゾーン情報を保持する型です。世界中どこからINSERTしてもUTCに換算して保存され、SELECT時はクライアントのタイムゾーンに自動変換されます。`timestamp`（タイムゾーンなし）だと「東京の23時」「ロンドンの23時」が同じ値になって混乱するので、原則 `timestamptz` を使います。

---

## 4. 書籍管理アプリのデータベース設計

### 4.1 books テーブルの設計

書籍管理アプリで必要なデータを整理し、テーブルを設計しましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <div style="border: 2px solid #3ecf8e; border-radius: 10px; overflow: hidden; max-width: 450px; margin: 0 auto; box-shadow: 0 4px 16px rgba(62,207,142,0.12);">
    <div style="background: #065f46; color: white; padding: 12px 16px; font-weight: 700; font-size: 15px; text-align: center;">📚 books</div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0; background: #f0fdf4;">
      <span style="color: #f59e0b; font-weight: 700;">🔑</span> id <span style="color: #94a3b8; float: right;">uuid (PK, auto)</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      title <span style="color: #dc2626; font-size: 10px; font-weight: 600;">NOT NULL</span> <span style="color: #94a3b8; float: right;">text</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      author <span style="color: #dc2626; font-size: 10px; font-weight: 600;">NOT NULL</span> <span style="color: #94a3b8; float: right;">text</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      publisher <span style="color: #94a3b8; float: right;">text</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      published_date <span style="color: #94a3b8; float: right;">date</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      rating <span style="color: #2563eb; font-size: 10px; font-weight: 600;">CHECK 1-5</span> <span style="color: #94a3b8; float: right;">integer</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      status <span style="color: #2563eb; font-size: 10px; font-weight: 600;">DEFAULT 'want_to_read'</span> <span style="color: #94a3b8; float: right;">text</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      notes <span style="color: #94a3b8; float: right;">text</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      cover_url <span style="color: #94a3b8; float: right;">text</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0;">
      created_at <span style="color: #2563eb; font-size: 10px; font-weight: 600;">DEFAULT NOW()</span> <span style="color: #94a3b8; float: right;">timestamptz</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px;">
      updated_at <span style="color: #2563eb; font-size: 10px; font-weight: 600;">DEFAULT NOW()</span> <span style="color: #94a3b8; float: right;">timestamptz</span>
    </div>
  </div>
</div>

### 4.2 各カラムの設計理由

各カラム（列）がなぜ必要で、なぜそのデータ型を選んだのか、一つずつ解説します。

#### `id` - uuid (PK, auto-generated)

```sql
-- id              : カラム名（このテーブル内での識別用、慣習的にidという名前を使う）
-- uuid            : データ型。128ビットのUUID（世界中で重複しないランダム文字列）を格納
-- DEFAULT ...     : INSERTで値が指定されなかったときに自動で入る値
-- gen_random_uuid(): Postgresが標準で持つ「新しいUUIDを1つ作る」関数
-- PRIMARY KEY     : このカラムを主キーにする宣言（NOT NULL かつ UNIQUE が自動付与される）
id uuid DEFAULT gen_random_uuid() PRIMARY KEY
```

- **役割:** 各書籍を一意に識別するための主キー
- **型の理由:** UUID を使うことで、グローバルに一意な ID を自動生成できる。将来的にユーザー認証を追加して複数ユーザーのデータを扱う際にも衝突しない
- **`DEFAULT gen_random_uuid()`:** INSERT 時に自動的に UUID が生成されるため、フロントエンドから ID を指定する必要がない
- **`PRIMARY KEY`:** この列が主キーであることを示す

#### `title` - text (NOT NULL)

```sql
-- title    : カラム名（書籍のタイトル）
-- text     : 可変長の文字列型。長さ制限なし（日本語もOK）
-- NOT NULL : 値が空（NULL）の行は許可しない、という制約
title text NOT NULL
```

- **役割:** 書籍のタイトル
- **型の理由:** タイトルの長さは予測できないため、可変長の `text` を使用
- **`NOT NULL`:** 書籍にタイトルがないのはありえないので、必須項目にする。NULL（空）でのINSERTを防ぐ

#### `author` - text (NOT NULL)

```sql
-- author   : カラム名（著者名）
-- text     : 文字列型
-- NOT NULL : 必須項目（空欄での登録を防ぐ）
author text NOT NULL
```

- **役割:** 著者名
- **型の理由:** 著者名も長さが予測できないため `text` を使用
- **`NOT NULL`:** 著者不明の書籍は「著者不明」と入力する想定。NULL は避ける
- **設計メモ:** 本格的なアプリでは著者を別テーブルに分離して多対多のリレーションにしますが、学習用アプリなのでシンプルにテキストで保持します

#### `publisher` - text

```sql
-- publisher : カラム名（出版社名）
-- text      : 文字列型
-- NOT NULL を付けないので NULL を許可（任意項目になる）
publisher text
```

- **役割:** 出版社名
- **型の理由:** 出版社名も長さが予測できないため `text`
- **NULL 許可:** 出版社が分からない、または入力しない場合もあるため、NULL を許可（任意項目）

#### `published_date` - date

```sql
-- published_date : カラム名（出版日）
-- date           : 「年月日」だけを格納する型。時分秒は持たない
-- NOT NULLなし   : NULL許可（出版日不明でも登録できる）
published_date date
```

- **役割:** 出版日
- **型の理由:** 年月日のみで十分なので `date` 型を使用（時刻は不要）
- **NULL 許可:** 出版日が不明な場合もあるため、NULL を許可
- **注意:** `timestamp` ではなく `date` を使うのは、出版日に時分秒の情報は必要ないため

#### `rating` - integer (1-5)

```sql
-- rating  : カラム名（評価値）
-- integer : 整数型（-2147483648〜2147483647までの範囲）
-- CHECK ( ... ) : 値の妥当性を検査する制約。条件を満たさない値はINSERT/UPDATE時に拒否
-- rating >= 1 AND rating <= 5 : 1以上かつ5以下のみ許可、という条件式
rating integer CHECK (rating >= 1 AND rating <= 5)
```

- **役割:** 自分の評価（星1〜5つ）
- **型の理由:** 1〜5 の整数値なので `integer` が最適
- **`CHECK` 制約:** データベースレベルで 1〜5 の範囲を強制する。これにより、フロントエンドのバグで不正な値が入ることを防げる
- **NULL 許可:** まだ読んでいない本は評価できないため、NULL を許可

#### `status` - text ('reading' | 'completed' | 'want_to_read')

```sql
-- status : カラム名（読書状態）
-- text   : 文字列型
-- DEFAULT 'want_to_read' : INSERT時に値が指定されなければ 'want_to_read' を入れる
-- CHECK (status IN (...)) : 指定された3つの値のいずれかでないと弾く制約
-- IN は「リストの中に含まれるか」を判定するSQL演算子
status text DEFAULT 'want_to_read' CHECK (status IN ('reading', 'completed', 'want_to_read'))
```

- **役割:** 読書状態の管理
- **型の理由:** 決まった値のいずれかなので `text` + `CHECK` 制約で管理
- **`DEFAULT 'want_to_read'`:** 新しく登録した本はデフォルトで「読みたい」状態にする
- **`CHECK` 制約:** 許可された3つの値以外は登録できないようにする
- **各値の意味:**
  - `reading`: 現在読んでいる
  - `completed`: 読了済み
  - `want_to_read`: 読みたい

> **なぜ `enum` 型ではなく `text` + `CHECK` か？**
> PostgreSQL には `enum` 型がありますが、後から値を削除するのが難しいという問題があります。`text` + `CHECK` であれば、将来的に `CHECK` 制約を変更するだけで値を追加・変更・削除できます。

#### `notes` - text

```sql
-- notes : カラム名（読書メモ）
-- text  : 文字列型（長文OK）
-- NULL許可（メモは任意）
notes text
```

- **役割:** 読書メモ・感想
- **型の理由:** 長文を格納できる `text` を使用
- **NULL 許可:** メモは任意項目

#### `cover_url` - text

```sql
-- cover_url : カラム名（表紙画像のURL）
-- text      : URL文字列を格納
-- NULL許可（画像未設定でもOK）
cover_url text
```

- **役割:** 書籍の表紙画像の URL
- **型の理由:** URL は文字列なので `text`
- **NULL 許可:** 画像がない場合もある
- **設計メモ:** 将来的に Supabase Storage を使って画像をアップロードする機能を追加する際、ここに Storage の URL を格納する

#### `created_at` - timestamptz

```sql
-- created_at  : カラム名（作成日時。慣習的にこの名前を使う）
-- timestamptz : タイムゾーン付きの日時型（worldwide で正しい時刻を扱える）
-- DEFAULT NOW() : INSERT時に値がなければ「現在時刻」が自動で入る
-- NOW() : 現在の日時を返すPostgres組み込み関数
created_at timestamptz DEFAULT NOW()
```

- **役割:** データが作成された日時
- **型の理由:** タイムゾーン情報を含む `timestamptz`（timezone 付き timestamp）を使用。世界中どこからアクセスしても正しい時刻が記録される
- **`DEFAULT NOW()`:** INSERT 時に自動的に現在時刻が設定される

#### `updated_at` - timestamptz

```sql
-- updated_at  : カラム名（更新日時）
-- timestamptz : タイムゾーン付きの日時型
-- DEFAULT NOW() : 作成時の値として現在時刻を入れる
-- UPDATE時に自動で書き換えるためには「トリガー」を別途定義する必要あり（後述）
updated_at timestamptz DEFAULT NOW()
```

- **役割:** データが最後に更新された日時
- **型の理由:** `created_at` と同じく `timestamptz`
- **`DEFAULT NOW()`:** INSERT 時に自動的に現在時刻が設定される
- **注意:** UPDATE 時に自動更新するにはトリガーが必要（後述）

### 4.3 SQL でのテーブル作成

以下の SQL で books テーブルを作成します。Supabase の SQL Editor で実行してください。

**ステップ 1: SQL Editor を開く**

Supabase ダッシュボードの左メニューから「SQL Editor」をクリックします。

> **Supabase Studio（管理画面）とSQLの対応:**
> Supabase の管理画面は内部的には毎回SQLを実行しています。「Table Editor で列を追加する」ボタンを押すと、裏で `ALTER TABLE ... ADD COLUMN ...` が走るだけ。GUIに慣れたら、SQL Editor で直接書いた方が早いと感じるはずです。

**ステップ 2: 新しいクエリを作成**

「New query」ボタンをクリックし、以下の SQL をコピー＆ペーストします。

> **▼ このコードがやること（先に日本語で）:** このアプリの中心となる **books（書籍）テーブルを新しく作る**SQLです。`CREATE TABLE` で「どんな列（カラム）を、どんなデータ型・制約で持つか」を一気に定義します。あわせて、行が更新されるたびに `updated_at`（更新日時）を自動で書き換える**トリガー**も用意します。長く見えますが、ほとんどが各列の説明コメントなので、まずはコピペで実行してOKです。

```sql
-- ============================================================================
-- books テーブルの作成 — 書籍管理アプリのメインテーブル
-- ----------------------------------------------------------------------------
-- SQL の構文: CREATE TABLE テーブル名 ( カラム定義1, カラム定義2, ... );
-- カラム定義: 「カラム名 データ型 制約」の3つを並べて書く。
-- 行末のセミコロン ; が SQL文の終わり。
-- -- (ハイフン2つ) で始まる行はコメント（実行時に無視される）。
-- ============================================================================

-- CREATE TABLE : 新しいテーブルを作るSQL文の開始キーワード
-- books        : 作成するテーブル名（複数形にするのが慣習）
-- (...)        : この中にカラム定義を並べる
CREATE TABLE books (

  -- (1) 主キー(PRIMARY KEY): レコードを一意に識別する列。
  --     id                   : このテーブル内でのカラム名
  --     uuid                 : 文字列の一種で、世界中で重複しないIDを表す型（128ビット）
  --     gen_random_uuid()    : Postgres組み込み関数。新しいUUIDを1つ生成する
  --     DEFAULT ...          : INSERT時にこの値が省略されたら自動で入る初期値
  --     PRIMARY KEY          : 主キー宣言。NULL不可・重複不可になる
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

  -- (2) 書籍情報（必須）
  --     title    : タイトル列。カラム名
  --     text     : 可変長の文字列（長さ制限なし）。MySQL の VARCHAR に相当
  --     NOT NULL : 「NULL（値なし）を許可しない」制約。空のINSERTを拒否
  title text NOT NULL,
  -- author : 著者名。同じく text NOT NULL（必須項目）
  author text NOT NULL,

  -- (3) 書籍情報（任意）
  --     publisher : 出版社名のカラム
  --     NOT NULL を付けないので、INSERT時に省略可能 → NULLが入る
  publisher text,
  --     published_date : 出版日のカラム
  --     date : 「年月日」だけを保存する型。時刻は持たない（軽量）
  published_date date,

  -- (4) 読書管理
  --     rating  : 評価値（1〜5の星）のカラム
  --     integer : 整数型（負の値も入るが、CHECK制約で1〜5に絞る）
  --     CHECK ( ... ) : 値が条件を満たさなければINSERT/UPDATEを拒否する制約
  --     ここでは「rating は 1以上かつ5以下」を強制する（5を超える値は弾かれる）
  rating integer CHECK (rating >= 1 AND rating <= 5),

  --     status : 読書状態のカラム
  --     text   : 文字列で 'reading'/'completed'/'want_to_read' のいずれかを格納
  --     DEFAULT 'want_to_read' : INSERT時に省略されたら 'want_to_read' を入れる
  --     CHECK (status IN ('reading', 'completed', 'want_to_read'))
  --       → これらの値以外は登録不可（typoや不正値を防ぐ）
  --       → IN はリストにマッチするかを判定する演算子
  status text DEFAULT 'want_to_read'
    CHECK (status IN ('reading', 'completed', 'want_to_read')),

  -- notes : 読書メモ。任意（NULL可能）
  notes text,

  -- (5) 画像URL
  -- cover_url : 表紙画像のURLを格納。Supabase Storage 等のURLを想定
  cover_url text,

  -- (6) タイムスタンプ
  --     created_at : 作成日時のカラム
  --     timestamptz : 「タイムゾーン付きの日時」を表す型（推奨）
  --     NOW()       : 現在の日時を返す関数
  --     DEFAULT NOW() で「INSERT時に自動で現在時刻が入る」
  created_at timestamptz DEFAULT NOW(),
  -- updated_at : 更新日時。INSERT時はNOW()が入り、UPDATE時はトリガーで更新（後述）
  updated_at timestamptz DEFAULT NOW()
);
-- 上のセミコロンで CREATE TABLE 文が終わる

-- ▼ 実行結果（成功時、Supabase SQL Editor の出力）
--   Success. No rows returned
--   → CREATE 系の文は「行を返さない」ので「成功・行なし」と表示される。


-- ============================================================================
-- updated_at を自動更新するトリガー
-- ----------------------------------------------------------------------------
-- 「レコードが UPDATE されるたびに updated_at を現在時刻にする」仕組みを
-- DBレベルで実現する。アプリ側でいちいち書かなくていい。
-- ============================================================================

-- (1) まず「呼ばれた時に動く関数」を定義する。
--     CREATE OR REPLACE FUNCTION:
--       同名の関数が既にあれば置き換える、無ければ新規作成。
--       何度実行しても安全（冪等性=べきとうせい）。
--     update_updated_at_column : この関数の名前（自分で命名）
--     () : 引数なし
--     RETURNS TRIGGER:
--       戻り値の型がトリガー専用の特殊な値であることを示す。
--     AS $$ ... $$:
--       関数本体を囲む区切り。シングルクォートを多用するときに便利。
--     LANGUAGE plpgsql:
--       関数本体は PL/pgSQL（Postgres の手続き型言語）で書く宣言。
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
-- BEGIN ... END; : 関数本体の処理ブロック
BEGIN
  -- NEW は「これから書き込まれる新しい行」の擬似変数
  -- そのレコードの updated_at を現在時刻に書き換える
  -- NOW() で現在のタイムスタンプを取得し代入
  NEW.updated_at = NOW();
  -- 書き換えた NEW を返すことでDBに反映される
  -- RETURN を省略するとUPDATEがキャンセル扱いになるので必須
  RETURN NEW;
END;
-- $$ で関数本体の終わり
$$ LANGUAGE plpgsql;

-- (2) 上の関数を「books テーブルが UPDATE される直前」に呼ぶよう紐付ける
--     CREATE TRIGGER : トリガー（特定イベントで自動実行される処理）を作成
--     update_books_updated_at : トリガー名（自由命名）
--     BEFORE UPDATE: 「UPDATE実行の直前に呼ぶ」タイミング指定
--       (AFTER UPDATE だと「実行後に呼ぶ」)
--     ON books     : booksテーブルを対象にする
--     FOR EACH ROW : 「行ごとに1回呼ぶ」（複数行同時更新時も全行に効く）
--     EXECUTE FUNCTION ...: 実行する関数の指定
CREATE TRIGGER update_books_updated_at
  BEFORE UPDATE ON books
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

-- ▼ これ以降、UPDATE文を実行するたびに自動で updated_at が NOW() に書き換わる。
--   例: UPDATE books SET title='新タイトル' WHERE id='...';
--   → 自分で updated_at を指定しなくても、自動的に現在時刻が入る。

-- ============================================
-- テーブルとカラムにコメントを追加
-- Dashboard での表示が分かりやすくなる
-- ============================================

-- COMMENT ON TABLE テーブル名 IS '説明文'; でテーブル全体に注釈を付ける
COMMENT ON TABLE books IS '書籍管理テーブル';
-- COMMENT ON COLUMN テーブル.カラム名 IS '説明文'; でカラムに注釈を付ける
COMMENT ON COLUMN books.id IS '書籍ID（自動生成）';
COMMENT ON COLUMN books.title IS '書籍タイトル';
COMMENT ON COLUMN books.author IS '著者名';
COMMENT ON COLUMN books.publisher IS '出版社';
COMMENT ON COLUMN books.published_date IS '出版日';
COMMENT ON COLUMN books.rating IS '評価（1-5）';
COMMENT ON COLUMN books.status IS '読書状態（reading/completed/want_to_read）';
COMMENT ON COLUMN books.notes IS 'メモ・感想';
COMMENT ON COLUMN books.cover_url IS '表紙画像URL';
COMMENT ON COLUMN books.created_at IS '作成日時';
COMMENT ON COLUMN books.updated_at IS '更新日時';
```

#### ▼ コードを1つずつ分解して解説

上の SQL は長く見えますが、「①テーブルを作る」「②自動更新の仕組みを作る」「③注釈を付ける」の3つに分かれています。SQL を初めて見る人向けに、塊ごとに区切って解説します。

##### 解説1: CREATE TABLE で「表のひな型」を作る

```sql
CREATE TABLE books (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  title text NOT NULL,
  author text NOT NULL,
```

- `CREATE TABLE books ( ... )` は「`books` という名前の新しいテーブル（表）を作る」という命令です。`( )` の中に、列（カラム）を1つずつカンマ区切りで並べます。
- 1つの列は「**列名 → データ型 → 制約**」の3つの順で書きます。`title text NOT NULL` なら「`title` という名前」「文字列型」「空を許さない」という意味です。
- `uuid` は「世界中で重複しない長いID」を入れる型、`text` は「長さ制限のない文字列」を入れる型です。
- `DEFAULT gen_random_uuid()` は「INSERT時に値を指定しなければ、自動でUUIDを1つ作って入れる」初期値の指定です。

> **用語:** **カラム（列）** = 表の縦の項目（id・title など）。**データ型** = その列に入れられる値の種類（数値・文字列・日付など）。**制約（constraint）** = 「こういう値は入れてはダメ」というルール。

---

##### 解説2: NOT NULL / DEFAULT / CHECK の3つの制約

```sql
  publisher text,
  published_date date,
  rating integer CHECK (rating >= 1 AND rating <= 5),
  status text DEFAULT 'want_to_read'
    CHECK (status IN ('reading', 'completed', 'want_to_read')),
```

- `NOT NULL` を**付けない**列（`publisher`・`published_date` など）は「空（NULL）でもOK＝任意項目」になります。逆に `NOT NULL` を付けると必須項目です。
- `CHECK ( 条件 )` は「条件を満たさない値は登録させない」制約です。`rating >= 1 AND rating <= 5` で「1以上5以下の整数しか入れられない」を強制します。
- `DEFAULT 'want_to_read'` は「INSERT時に `status` を省略したら自動で `'want_to_read'` を入れる」という初期値です。
- `IN ('reading', 'completed', 'want_to_read')` は「この3つの値のどれかであること」を判定します。これでスペルミスや想定外の値の登録を防げます。

> **用語:** **NULL（ヌル）** = 「値が入っていない（未入力）」状態のこと。空文字 `''` とは別物。**IN** = 「カッコ内のリストのどれかに一致するか」を調べるSQL演算子。

---

##### 解説3: トリガー関数で「更新時に時刻を自動で書き換える」

```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

- これは「行が更新される直前に呼ばれる小さな処理（関数）」を作る部分です。中身は「これから保存する行の `updated_at` を、今の時刻 `NOW()` に書き換える」だけです。
- `NEW` は「これから書き込まれる新しい行」を表す特別な変数です。`NEW.updated_at = NOW()` でその行の更新日時を現在時刻にしています。
- `RETURN NEW` で書き換えた行を返すことで、DBに反映されます（これを省くと更新がキャンセル扱いになります）。
- `CREATE OR REPLACE` なので、同じ名前の関数が既にあっても上書きされ、何度実行しても安全です。

> **用語:** **関数（FUNCTION）** = DB内に保存しておき、必要なときに呼び出せる処理のまとまり。**NOW()** = 「現在の日時」を返すPostgreSQLの組み込み関数。

---

##### 解説4: トリガーで関数を「UPDATEの直前」に紐付ける

```sql
CREATE TRIGGER update_books_updated_at
  BEFORE UPDATE ON books
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

- `CREATE TRIGGER` は「あるイベントが起きたら、自動で関数を呼ぶ」仕掛けを作る命令です。
- `BEFORE UPDATE ON books` は「`books` テーブルが UPDATE される**直前**に発動する」というタイミング指定です。
- `FOR EACH ROW` は「更新される行1つごとに1回呼ぶ」という意味です。10行まとめて更新すれば10回呼ばれます。
- `EXECUTE FUNCTION update_updated_at_column()` で、解説3で作った関数を実行します。これ以降、`updated_at` を自分で指定しなくても自動で最新時刻になります。

> **用語:** **トリガー（trigger）** = 「DBで特定の操作（INSERT/UPDATE/DELETE）が起きたら自動で動く仕掛け」。引き金（trigger）のように、イベントをきっかけに処理が走る。

---

**ステップ 3: SQL を実行**

SQL Editor の右下にある「Run」ボタン（または `Ctrl + Enter` / `Cmd + Enter`）をクリックして実行します。

```
Success. No rows returned.
```

と表示されれば成功です。

**ステップ 4: テーブルの確認**

左メニューの「Table Editor」をクリックすると、作成した `books` テーブルが表示されているはずです。テーブル名をクリックすると、カラムの一覧やデータ（まだ空）が確認できます。

### 4.4 Supabase Dashboard での GUI 操作手順（代替手段）

SQL を使わずに、Dashboard の GUI でテーブルを作成することもできます。ここではその手順を説明します。

> **推奨:** SQL での作成を推奨しますが、GUI での操作も知っておくと便利です。

**ステップ 1: Table Editor を開く**

左メニューの「Table Editor」をクリックします。

**ステップ 2: 新しいテーブルを作成**

「Create a new table」ボタンをクリックします。

**ステップ 3: テーブル名の入力**

- **Name:** `books` と入力
- **Description:** `書籍管理テーブル` と入力（任意）
- **Enable Row Level Security (RLS):** チェックを入れる（後で設定）

**ステップ 4: カラムの追加**

デフォルトで `id`（uuid, Primary Key）と `created_at`（timestamptz）が用意されています。残りのカラムを以下の手順で追加します:

1. 「Add column」ボタンをクリック
2. 以下の情報を入力:

```
┌──────────────────────────────────────────────────────────┐
│  Column Name: title                                      │
│  Type: text                                              │
│  Default Value: (空欄)                                    │
│  Primary: [ ]                                            │
│  Nullable: [ ] ← チェックを外す（NOT NULL にする）         │
│  Unique: [ ]                                             │
│  [Save]                                                  │
└──────────────────────────────────────────────────────────┘
```

3. 同様に、残りのカラムも一つずつ追加:
   - `author` (text, NOT NULL)
   - `publisher` (text, Nullable)
   - `published_date` (date, Nullable)
   - `rating` (int4, Nullable)
   - `status` (text, Default: 'want_to_read')
   - `notes` (text, Nullable)
   - `cover_url` (text, Nullable)
   - `updated_at` (timestamptz, Default: now())

4. すべてのカラムを追加したら「Save」ボタンをクリック

> **注意:** GUI では CHECK 制約やトリガーを設定できないため、別途 SQL Editor で以下を実行する必要があります:

> **▼ このコードがやること（先に日本語で）:** GUI（画面ボタン操作）でテーブルを作った場合に**足りない設定を後から付け足す**SQLです。`ALTER TABLE ... ADD CONSTRAINT` で「rating は1〜5」「status は3つの値だけ」という**CHECK制約**を追加し、さらに `updated_at` 自動更新の**トリガー**も作ります。GUI操作の仕上げとして実行する、と捉えてください。

```sql
-- ALTER TABLE : 既存テーブルの定義を変更するSQL文
-- books       : 対象テーブル名
-- ADD CONSTRAINT 制約名 ... : 新しい制約を追加する
-- books_rating_check : 制約に付ける名前（テーブル名_カラム名_check が慣習）
-- CHECK (rating >= 1 AND rating <= 5) : rating が 1〜5 の範囲のみ許可
ALTER TABLE books
  ADD CONSTRAINT books_rating_check CHECK (rating >= 1 AND rating <= 5);

-- 同様に status の値を 3つに限定する制約を追加
-- IN ('reading', 'completed', 'want_to_read') : このリストの値以外は弾く
ALTER TABLE books
  ADD CONSTRAINT books_status_check CHECK (status IN ('reading', 'completed', 'want_to_read'));

-- updated_at 自動更新トリガーの追加（仕組みは前述と同じ）
-- CREATE OR REPLACE FUNCTION : 既存なら置換、なければ作成
CREATE OR REPLACE FUNCTION update_updated_at_column()
-- RETURNS TRIGGER : トリガー専用関数として宣言
RETURNS TRIGGER AS $$
BEGIN
  -- NEW.updated_at : これから書き込まれる行の updated_at を…
  -- NOW()          : 現在時刻に書き換える
  NEW.updated_at = NOW();
  -- 書き換えた行を返す（これでDBに反映される）
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- トリガーの作成: UPDATE 直前にこの関数を呼ぶよう紐付け
CREATE TRIGGER update_books_updated_at
  BEFORE UPDATE ON books       -- UPDATE 文の直前に発火
  FOR EACH ROW                  -- 行ごとに1回呼ぶ
  EXECUTE FUNCTION update_updated_at_column();  -- 上で作った関数を実行
```

---

## 5. Row Level Security (RLS)

### 5.1 RLS とは

**Row Level Security（RLS: 行レベルセキュリティ）** とは、PostgreSQL の機能で、テーブルの各行に対して「誰がどの行にアクセスできるか」を制御する仕組みです。

**なぜ RLS が必要か？**

Supabase はフロントエンド（ブラウザ）から直接データベースにアクセスします。つまり、API キー（anon key）はブラウザの JavaScript コードに埋め込まれ、誰でも見ることができます。

```
通常のサーバー構成:
  ブラウザ → API サーバー（ここでアクセス制御）→ データベース

Supabase の構成:
  ブラウザ → Supabase API（ここに RLS でアクセス制御）→ データベース
```

RLS がなければ、anon key を知っている人は誰でもすべてのデータを読み書きできてしまいます。RLS を有効にすることで、「どのユーザーがどのデータにアクセスできるか」をデータベースレベルで制御できます。

> **RLSを無効化したら何が起きる？**
>
> RLSがOFFの状態でテーブルを世に公開すると、anon key（フロントに埋め込まれている公開キー）を知る誰でもが
> 1. 全データを SELECT で読める（個人情報の漏えい）
> 2. 任意のデータを INSERT できる（スパム書き込み）
> 3. 既存データを UPDATE で改ざんできる
> 4. DELETE で全削除もできる（破壊行為）
>
> という状態になります。**本番環境ではRLSは必須**と覚えてください。ダッシュボードにも「Unprotected」と警告が出ます。

### 5.2 RLS の仕組み（図解）

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- Users Row -->
  <div style="display: flex; justify-content: center; gap: 40px; margin-bottom: 12px;">
    <div style="background: #eff6ff; border: 2px solid #3b82f6; border-radius: 10px; padding: 12px 20px; text-align: center;">
      <div style="font-size: 18px;">👤</div>
      <div style="font-weight: 700; color: #1e40af; font-size: 13px;">User A Request</div>
    </div>
    <div style="background: #fef3c7; border: 2px solid #f59e0b; border-radius: 10px; padding: 12px 20px; text-align: center;">
      <div style="font-size: 18px;">👤</div>
      <div style="font-weight: 700; color: #92400e; font-size: 13px;">User B Request</div>
    </div>
  </div>
  <!-- Arrow Down -->
  <div style="text-align: center; font-size: 18px; color: #64748b; margin: 4px 0;">↓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;↓</div>
  <!-- RLS Policy Evaluation -->
  <div style="background: #fdf2f8; border: 2px solid #ec4899; border-radius: 12px; padding: 14px 24px; text-align: center; margin: 0 auto 12px; max-width: 380px;">
    <div style="font-size: 18px;">🛡️</div>
    <div style="font-weight: 700; color: #9d174d; font-size: 14px;">RLS Policy Evaluation</div>
    <div style="font-size: 11px; color: #64748b; margin-top: 4px;">Policy: user can access own data only</div>
  </div>
  <!-- Arrow Down with labels -->
  <div style="display: flex; justify-content: center; gap: 80px; margin: 4px 0;">
    <div style="text-align: center; font-size: 12px; color: #3b82f6; font-weight: 600;">↓ User A data only</div>
    <div style="text-align: center; font-size: 12px; color: #f59e0b; font-weight: 600;">↓ User B data only</div>
  </div>
  <!-- Books Table -->
  <div style="border: 2px solid #3ecf8e; border-radius: 10px; overflow: hidden; margin-top: 8px;">
    <div style="background: #065f46; color: white; padding: 10px 16px; font-weight: 700; font-size: 14px; text-align: center;">📚 books table</div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0; background: #eff6ff; display: flex; justify-content: space-between; align-items: center;">
      <span>id: 1, user_id: <strong style="color: #1e40af;">A</strong>, title: Book A</span>
      <span style="background: #3b82f6; color: white; padding: 2px 8px; border-radius: 4px; font-size: 10px; font-weight: 600;">User A</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #e2e8f0; background: #fef9c3; display: flex; justify-content: space-between; align-items: center;">
      <span>id: 2, user_id: <strong style="color: #92400e;">B</strong>, title: Book B</span>
      <span style="background: #f59e0b; color: white; padding: 2px 8px; border-radius: 4px; font-size: 10px; font-weight: 600;">User B</span>
    </div>
    <div style="padding: 8px 16px; font-size: 12px; background: #eff6ff; display: flex; justify-content: space-between; align-items: center;">
      <span>id: 3, user_id: <strong style="color: #1e40af;">A</strong>, title: Book C</span>
      <span style="background: #3b82f6; color: white; padding: 2px 8px; border-radius: 4px; font-size: 10px; font-weight: 600;">User A</span>
    </div>
  </div>
</div>

### 5.3 書籍管理アプリでの RLS 設定（簡易版）

本チュートリアルではまだ認証機能を実装していないため、まずは **誰でも全データにアクセスできる** シンプルなポリシーを設定します。認証機能の実装後に、より厳密なポリシーに変更します。

**SQL Editor で以下を実行してください:**

> **▼ このコードがやること（先に日本語で）:** books テーブルに**「誰がどの行を操作できるか」のルール（RLS：行レベルセキュリティ）**を設定するSQLです。まず `ENABLE ROW LEVEL SECURITY` で防御を ON にし、次に `CREATE POLICY` で SELECT/INSERT/UPDATE/DELETE それぞれの**許可ルール**を1つずつ追加します。RLS は「何も書かなければ全部拒否、書いたぶんだけ許可」のホワイトリスト方式です。ここではまだ認証がないので、いったん**全員に全操作を許可（開発用）**にしています。

```sql
-- ============================================================================
-- RLS（Row Level Security）の設定
-- ----------------------------------------------------------------------------
-- 「テーブルの行ごとに、誰がアクセスできるかを決める」セキュリティ機能。
-- Supabase ではテーブル作成時の状態によってRLSの初期値が変わる。
-- ALTER TABLE で有効化し、CREATE POLICY で「許可ルール」を1つずつ追加していく。
-- ポリシーは「ホワイトリスト方式」: 何も設定しないと全部拒否、書いたぶんだけ許可される
-- ============================================================================

-- (1) books テーブルの RLS を有効化する
--     ALTER TABLE          : 既存テーブルの設定を変更するSQL文
--     books                : 対象テーブル
--     ENABLE ROW LEVEL SECURITY : RLSを ON にする
--     ※ RLSを有効にしたあと、ポリシーを1つも作っていないと「全部拒否」になる
--       （SELECTしてもデータが空配列で返るのに気づかずハマる定番ポイント）
ALTER TABLE books ENABLE ROW LEVEL SECURITY;


-- (2) SELECT（読み取り）を全員に許可するポリシー
--     CREATE POLICY "ポリシー名" ON テーブル名
--     "Allow public read access" : ポリシー名（自由命名、空白OK）
--     ON books      : booksテーブルに適用
--     FOR SELECT    : どの操作（SELECT/INSERT/UPDATE/DELETE/ALL）に効くか
--     USING (条件)  : どの「既存の行」がこの操作に使えるかを決めるブール式
--     true は「常に真」 → 全行に対して許可するという意味
CREATE POLICY "Allow public read access"
  ON books
  FOR SELECT
  USING (true);


-- (3) INSERT（新規作成）を全員に許可するポリシー
--     "Allow public insert access" : ポリシー名
--     FOR INSERT    : INSERT操作向けポリシー
--     INSERTでは「これから書き込まれる新行」が対象なので USING ではなく
--     WITH CHECK を使う。
--     WITH CHECK (true) → どんな値の行でも書き込みOK
CREATE POLICY "Allow public insert access"
  ON books
  FOR INSERT
  WITH CHECK (true);


-- (4) UPDATE（更新）を全員に許可するポリシー
--     FOR UPDATE   : UPDATE操作向けポリシー
--     UPDATEは「既存の行を選んで」「新しい値で上書き」の2フェーズなので
--     USING（読み取り対象の判定）と WITH CHECK（書き込み内容の判定）の両方を書く。
CREATE POLICY "Allow public update access"
  ON books
  FOR UPDATE
  USING (true)         -- 全ての既存行を更新対象にできる
  WITH CHECK (true);   -- どんな新しい値でも書き込みOK


-- (5) DELETE（削除）を全員に許可するポリシー
--     FOR DELETE   : DELETE操作向けポリシー
--     DELETEは新規行を作らないので WITH CHECK は不要、USINGだけで判定する。
CREATE POLICY "Allow public delete access"
  ON books
  FOR DELETE
  USING (true);

-- ▼ 実行後の状態
--   このテーブルは「誰でもCRUD可能な開発用設定」になる。
--   本番運用や認証実装後は、この4つを DROP POLICY で削除し、
--   auth.uid() = user_id のような「自分のレコードだけ」ポリシーに差し替える。
```

#### ▼ コードを1つずつ分解して解説

RLS は「**まず防御をON → 許可ルール（ポリシー）を1つずつ足していく**」という流れです。塊ごとに見ていきましょう。

##### 解説1: ENABLE ROW LEVEL SECURITY で防御をONにする

```sql
ALTER TABLE books ENABLE ROW LEVEL SECURITY;
```

- `ALTER TABLE books` は「既存の `books` テーブルの設定を変更する」命令です（`CREATE` は新規作成、`ALTER` は変更）。
- `ENABLE ROW LEVEL SECURITY` で、そのテーブルの「行レベルセキュリティ（RLS）」をONにします。
- **重要な落とし穴:** RLSをONにした直後は「ポリシーが1つもない＝全部拒否」状態です。SELECTしてもエラーは出ず、ただ空配列 `[]` が返るだけなので、原因に気づきにくいです。

> **用語:** **RLS（Row Level Security）** = 「テーブルの行（レコード）1件ごとに、誰がアクセスできるかを決める」しくみ。ホワイトリスト方式（許可したものだけ通す）。

---

##### 解説2: CREATE POLICY で SELECT（読み取り）を許可する

```sql
CREATE POLICY "Allow public read access"
  ON books
  FOR SELECT
  USING (true);
```

- `CREATE POLICY "名前"` で許可ルールを1つ作ります。名前は自由で、空白を含めてもOKです。
- `ON books` は「`books` テーブルに対するルール」、`FOR SELECT` は「SELECT（読み取り）操作に効くルール」という指定です。
- `USING ( 条件 )` は「**どの既存の行が、この操作の対象になれるか**」を真偽（true/false）で判定する式です。
- `USING (true)` は「常に真＝すべての行を読み取り可能」という意味です（開発用の全許可）。

> **用語:** **ポリシー（policy）** = RLSにおける「許可ルール」1つ1つのこと。**USING** = 「すでにテーブルにある行のうち、どれを対象にできるか」を決める条件。

---

##### 解説3: INSERT は WITH CHECK で「書き込む値」を判定する

```sql
CREATE POLICY "Allow public insert access"
  ON books
  FOR INSERT
  WITH CHECK (true);
```

- `FOR INSERT` は「新規追加の操作に効くルール」です。
- INSERTは「これから書き込む新しい行」が対象なので、`USING`（既存行の判定）ではなく `WITH CHECK`（これから書く行の判定）を使います。
- `WITH CHECK (true)` は「どんな値の行でも書き込みOK」という意味です。

> **用語:** **WITH CHECK** = 「これから書き込もうとしている（新しい/更新後の）行が、登録を許される値かどうか」を判定する条件。INSERT・UPDATE で使う。

---

##### 解説4: UPDATE は USING と WITH CHECK の両方を使う

```sql
CREATE POLICY "Allow public update access"
  ON books
  FOR UPDATE
  USING (true)         -- 全ての既存行を更新対象にできる
  WITH CHECK (true);   -- どんな新しい値でも書き込みOK
```

- UPDATEは「①既存の行を選ぶ → ②新しい値で上書きする」の2段階なので、両方の判定が必要です。
- `USING (true)` で「すべての既存行を更新対象にできる」、`WITH CHECK (true)` で「上書き後の値は何でもOK」を表します。
- DELETE（解説では割愛しますが上のSQLの(5)）は新しい行を作らないため `WITH CHECK` は不要で、`USING` だけで「どの行を消せるか」を判定します。

> **用語:** **FOR SELECT / INSERT / UPDATE / DELETE** = そのポリシーが「どの操作に効くか」の指定。`FOR ALL` と書くと4つすべてに効く。

---

**各ポリシーの解説:**

| ポリシー名 | 対象操作 | `USING` | `WITH CHECK` | 意味 |
|-----------|---------|---------|--------------|------|
| Allow public read access | SELECT | `true` | - | すべての行を読み取り可能 |
| Allow public insert access | INSERT | - | `true` | すべての行を挿入可能 |
| Allow public update access | UPDATE | `true` | `true` | すべての行を更新可能 |
| Allow public delete access | DELETE | `true` | - | すべての行を削除可能 |

**`USING` と `WITH CHECK` の違い:**

- **`USING`**: 既存の行に対する条件（SELECT, UPDATE, DELETE で使用）。「どの行が見えるか」
- **`WITH CHECK`**: 新しく書き込まれる行に対する条件（INSERT, UPDATE で使用）。「どの行を書き込めるか」

### 5.4 認証実装後の RLS ポリシー（参考）

第7章で認証機能を追加した後は、以下のようなポリシーに変更します。参考として掲載しておきます。

> **▼ このコードがやること（先に日本語で）:** 認証（ログイン）を入れた後に使う、**「自分のデータだけ操作できる」厳しめのRLSポリシー**の見本です。鍵は `auth.uid() = user_id` という条件で、「いま**ログイン中のユーザーのID**」と「その行の持ち主」が一致する行だけを許可します。今はまだ実行せず、将来こう書き換えるのだという**完成イメージ**として読んでください（全行がコメントアウトされています）。

```sql
-- 認証済みユーザーが自分のデータのみアクセスできるポリシー
-- （第7章で実装予定。今は実行しないでください）

-- 既存のポリシーを削除（DROP POLICY: ポリシーを消すSQL）
-- DROP POLICY "Allow public read access" ON books;
-- DROP POLICY "Allow public insert access" ON books;
-- DROP POLICY "Allow public update access" ON books;
-- DROP POLICY "Allow public delete access" ON books;

-- ユーザーは自分のデータのみ SELECT 可能
-- auth.uid()  : Supabaseが提供する関数。現在ログインしているユーザーのUUIDを返す
-- user_id     : booksテーブルの「持ち主」を表すFKカラム（あとで追加予定）
-- = で両者が等しい行だけを許可する条件
-- CREATE POLICY "Users can view own books"
--   ON books
--   FOR SELECT
--   USING (auth.uid() = user_id);

-- ユーザーは自分の user_id でのみ INSERT 可能
-- INSERT時の「これから書く行」の user_id がログイン中ユーザーと一致するかを検査
-- CREATE POLICY "Users can insert own books"
--   ON books
--   FOR INSERT
--   WITH CHECK (auth.uid() = user_id);

-- ユーザーは自分のデータのみ UPDATE 可能
-- USING で「自分の行だけ」読み出し、WITH CHECK で「自分のIDのまま」書き込みを保証
-- CREATE POLICY "Users can update own books"
--   ON books
--   FOR UPDATE
--   USING (auth.uid() = user_id)
--   WITH CHECK (auth.uid() = user_id);

-- ユーザーは自分のデータのみ DELETE 可能
-- CREATE POLICY "Users can delete own books"
--   ON books
--   FOR DELETE
--   USING (auth.uid() = user_id);
```

---

## 6. Supabase Client のセットアップ

### 6.1 @supabase/supabase-js のインストール

プロジェクトのルートディレクトリで以下のコマンドを実行します。

```bash
# npm install : npmパッケージをプロジェクトに追加するコマンド
# @supabase/supabase-js : Supabase公式のJavaScript/TypeScript SDKパッケージ名
#                         "@組織名/パッケージ名" の形式（スコープ付きパッケージ）
npm install @supabase/supabase-js
```

正常にインストールされると、`package.json` の `dependencies` に追加されます。

```json
{
  "dependencies": {
    "@supabase/supabase-js": "^2.x.x"
  }
}
```

> **`^2.x.x` の意味:** キャレット `^` は「メジャーバージョン（先頭の数字）は固定、それ以下は最新を使う」の意味。`^2.0.0` なら `2.x.x` の範囲で最新版が入る。

### 6.2 環境変数の設定

Supabase に接続するために必要な情報を環境変数として設定します。環境変数を使うことで、API キーなどの秘密情報をコードにハードコードせずに管理できます。

**ステップ 1: Supabase Dashboard から情報を取得**

1. Supabase Dashboard にログイン
2. 対象プロジェクトを選択
3. 左メニューの「Settings」(歯車アイコン) をクリック
4. 「API」をクリック
5. 以下の2つの値をコピー:
   - **Project URL**: `https://xxxxxxxx.supabase.co`
   - **anon public key**: `eyJhbGciOiJIUzI1NiIs...`（長い文字列。JWT形式）

**ステップ 2: `.env.local` ファイルの作成**

プロジェクトのルートディレクトリに `.env.local` ファイルを作成します。

> **▼ このコードがやること（先に日本語で）:** Supabase への**接続情報（URLとanonキー）を環境変数として書いておく**ファイルの中身です。「変数名=値」を1行ずつ書きます。先頭の `NEXT_PUBLIC_` は「この値はブラウザにも渡してよい」という Next.js の合図です。`xxxxxxxx` の部分は自分のプロジェクトの実際の値に置き換えること、そして**このファイルは絶対にGitに公開しない**ことを必ず守ってください。

```bash
# .env.local
# Supabase の接続情報
# 「キー=値」の形式で1行1変数を書く（イコールの周りに空白を入れない）

# NEXT_PUBLIC_SUPABASE_URL : Supabase プロジェクトのAPIエンドポイント
#   NEXT_PUBLIC_ プレフィックスはNext.jsの約束で「ブラウザにも値を渡してOK」を意味する
NEXT_PUBLIC_SUPABASE_URL=https://xxxxxxxx.supabase.co
# NEXT_PUBLIC_SUPABASE_ANON_KEY : 匿名（anon）APIキー
#   ブラウザに埋め込まれて公開される前提のキー。RLSと組み合わせて安全に使う
NEXT_PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

> **重要な注意点:**
>
> 1. `xxxxxxxx` の部分は、あなたのプロジェクトの実際の値に置き換えてください
> 2. `NEXT_PUBLIC_` プレフィックスは Next.js で環境変数をブラウザ側で使うために必要です（このプレフィックスがない環境変数はサーバー側だけで参照可能）
> 3. `.env.local` は **絶対に Git にコミットしないでください**。`.gitignore` に含まれていることを確認しましょう
> 4. **service_role key（管理者キー）を `.env.local` に書く場合は、絶対に `NEXT_PUBLIC_` を付けてはいけません**。付けるとブラウザに漏れます

> **`NEXT_PUBLIC_` プレフィックスの仕組み:**
> Next.js は `process.env.XXX` を「ビルド時に値を埋め込む」処理をします。`NEXT_PUBLIC_` で始まる名前のものだけがブラウザ向けバンドルにも埋め込まれます。そうでないものはサーバー側（API Route や Server Component）でしか参照できません。これにより「ブラウザに渡したい変数」と「サーバーに隠したい変数」を分けて管理できます。

**ステップ 3: `.gitignore` に追加されていることを確認**

Next.js のプロジェクトを `create-next-app` で作成した場合、`.env.local` はデフォルトで `.gitignore` に含まれています。念のため確認しましょう。

```bash
# .gitignore に以下が含まれていることを確認
# .env*.local は「.envで始まり.localで終わる任意のファイル」をGit管理から除外する
.env*.local
```

もし含まれていなければ、`.gitignore` に以下を追加してください:

```
# 環境変数ファイル
.env*.local
```

### 6.3 Supabase クライアントの初期化

Supabase に接続するためのクライアントを作成します。

**ステップ 1: ファイルの作成**

`src/lib/supabase.ts` ファイルを作成します。

```bash
# mkdir -p : 中間ディレクトリも含めて作る（既にあってもエラーにならない）
# src/lib  : srcの下にlibディレクトリを作るパス
mkdir -p src/lib
```

**ステップ 2: クライアントコードの記述**

> **▼ このコードがやること（先に日本語で）:** アプリ全体から使い回す**Supabaseクライアント（DBへの接続窓口）を1個だけ作って共有する**ファイルです。`.env.local` に書いた**接続URLとanonキー**を読み込み、`createClient(...)` に渡して接続オブジェクトを生成します。値が未設定なら早めにエラーを出す仕組みも入れてあります。これを `export` しておくと、他のファイルから `import { supabase }` するだけでDB操作できるようになるのがポイントです。

```typescript
// ============================================================================
// ファイルパス: src/lib/supabase.ts
// 役割      : アプリ全体で使う「Supabaseクライアント」を1個だけ作って共有する
// ----------------------------------------------------------------------------
// このファイルを作っておくと、他のファイルから
//   import { supabase } from "@/lib/supabase";
// と書くだけでDB操作できるようになる。
// クライアントを1個にまとめる(=シングルトン化)ことで、接続の無駄遣いを防ぐ。
// ============================================================================

// (1) Supabase クライアント作成関数を取り込む
//     import { createClient } : 名前付きインポート（特定の関数だけ取り込む構文）
//     @supabase/supabase-js   : Supabase の公式SDK（npmパッケージ）の名前
import { createClient } from '@supabase/supabase-js';

// (2) DBスキーマから自動生成した型を取り込む
//     `import type` は「型情報だけ取り込み、実行時のJSには残さない」記法
//     Database はテーブル定義から自動生成された型（後述）
//     @ は src/ を表すパスエイリアス（tsconfig.json の paths で設定済み）
//     よって @/types/supabase は src/types/supabase.ts を指す
import type { Database } from '@/types/supabase';

// (3) 環境変数から接続情報を取得
//     process.env はNode.jsで環境変数を読むオブジェクト
//     Next.jsの場合、ビルド時に NEXT_PUBLIC_ で始まる値が埋め込まれる
//     NEXT_PUBLIC_SUPABASE_URL : .env.local で設定したSupabaseのURL
const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
// NEXT_PUBLIC_SUPABASE_ANON_KEY : .env.local で設定したanonキー（JWT文字列）
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

// (4) 環境変数が未設定の場合は早期にエラーを出す（Fail-Fast）
//     これを書かないと「なぜか動かない」と原因不明になりやすい。
//     エラー文に「.env.local を確認」と書いて初心者にやさしくする。
//     ! (論理否定) で「変数がundefined/空文字なら true」を判定
if (!supabaseUrl) {
  // throw new Error : エラーオブジェクトを投げる構文。実行を即停止する
  throw new Error(
    'NEXT_PUBLIC_SUPABASE_URL が設定されていません。.env.local ファイルを確認してください。'
  );
}

if (!supabaseAnonKey) {
  throw new Error(
    'NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。.env.local ファイルを確認してください。'
  );
}

// (5) Supabase クライアントを作って export する
//     createClient(url, anonKey) : Supabaseに接続するためのオブジェクトを返す関数
//     <Database> はジェネリクス型引数。これを渡しておくと
//       supabase.from('books').select('*') と書いたときに
//       VS Code が books の存在やカラム名を補完・検証してくれる。
//     export const にすることで、他のファイルから
//       import { supabase } from "@/lib/supabase";
//     で使えるようになる。
export const supabase = createClient<Database>(supabaseUrl, supabaseAnonKey);

// ▼ 使用例（別ファイルから）
//   import { supabase } from "@/lib/supabase";
//   const { data, error } = await supabase.from("books").select("*");
//   console.log(data); // Book[] 型（自動推論）
```

#### ▼ コードを1つずつ分解して解説

このファイルは「①必要なものを取り込む → ②接続情報を読む → ③値の有無をチェック → ④クライアントを作って共有する」という流れです。塊ごとに見ていきます。

##### 解説1: createClient と型をインポートする

```ts
import { createClient } from '@supabase/supabase-js';

import type { Database } from '@/types/supabase';
```

- `import { createClient } from '@supabase/supabase-js'` は、Supabaseの公式SDKから「クライアントを作る関数」だけを取り込む書き方です（名前付きインポート）。
- `import type { Database }` は「**型情報だけ**を取り込む」書き方で、実行時のJavaScriptには残りません。`Database` はテーブル定義から自動生成した型です（このあとの 6.4 で作ります）。
- `@/types/supabase` の `@` は「`src/` フォルダ」を指す近道（パスエイリアス）で、`src/types/supabase.ts` を意味します。

> **用語:** **import** = 別ファイルやパッケージの機能を「持ち込む」命令。**SDK** = ある機能を使うための道具一式（ここではSupabase操作用のライブラリ）。

---

##### 解説2: 環境変数から接続情報を読み込む

```ts
const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;
```

- `process.env.XXX` は「環境変数 XXX の値を読む」書き方です。さきほど `.env.local` に書いた値がここに入ってきます。
- URLとanonキーをコードに直接書かず環境変数から読むことで、秘密情報をソースコードに埋め込まずに済みます。
- `NEXT_PUBLIC_` で始まる名前なので、これらの値はブラウザ側でも参照できます（anonキーは公開前提のキーなのでOK）。

> **用語:** **環境変数** = プログラムの外側（OSや設定ファイル）から値を渡すしくみ。接続先やキーなど、環境ごとに変わる値を入れておく。

---

##### 解説3: 値が無ければ早めにエラーを出す（Fail-Fast）

```ts
if (!supabaseUrl) {
  throw new Error(
    'NEXT_PUBLIC_SUPABASE_URL が設定されていません。.env.local ファイルを確認してください。'
  );
}
```

- `!supabaseUrl` は「`supabaseUrl` が空（undefinedや空文字）なら true」という判定です。`!` は「〜でない」を表す否定記号です。
- 値が無いまま進むと「なぜか動かない」原因不明の状態になりがちです。そこで `throw new Error(...)` で**すぐに**処理を止め、分かりやすいメッセージを出します。
- このように「問題があれば早く失敗させる」考え方を Fail-Fast（フェイルファスト）と呼びます。

> **用語:** **throw（スロー）** = エラーを発生させて処理を中断する命令。**Error オブジェクト** = エラーの内容（メッセージなど）を入れる入れ物。

---

##### 解説4: クライアントを作って export し、共有する

```ts
export const supabase = createClient<Database>(supabaseUrl, supabaseAnonKey);
```

- `createClient(url, anonKey)` で「Supabaseに接続するためのオブジェクト」を1つ作ります。
- `<Database>` は型情報を渡す部分（ジェネリクス）です。これを渡しておくと、`supabase.from('books').select('*')` と書いたときにエディタがテーブル名やカラム名を補完・検証してくれます。
- `export const supabase = ...` で外部に公開しているので、他のファイルから `import { supabase } from '@/lib/supabase'` と書くだけで同じ接続を使い回せます（シングルトン）。

> **用語:** **export** = このファイルの値を「他ファイルから使えるように公開する」命令。**シングルトン** = 「インスタンスを1個だけ作って全体で共有する」設計。

---

**コードの解説:**

1. `createClient`: Supabase クライアントを作成する関数。`@supabase/supabase-js` からインポート
2. `Database`: TypeScript の型定義（次のセクションで生成）。これにより、テーブル名やカラム名の補完が効くようになる
3. `process.env.NEXT_PUBLIC_SUPABASE_URL`: 環境変数から Supabase の URL を取得
4. `process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY`: 環境変数から API キーを取得
5. エラーチェック: 環境変数が未設定の場合、分かりやすいエラーメッセージを表示

### 6.4 TypeScript 型の自動生成（supabase gen types）

Supabase CLI を使うと、データベースのスキーマ（テーブル定義）から TypeScript の型を自動生成できます。これにより、コード内でテーブル名やカラム名の入力補完が効くようになり、タイプミスを防げます。

**ステップ 1: Supabase CLI のインストール**

```bash
# -D は --save-dev の短縮形。開発時のみ使うパッケージとして package.json に記録
# supabase : CLIツールのパッケージ名
npm install -D supabase
```

**ステップ 2: Supabase CLI にログイン**

```bash
# npx : ローカルにインストールしたCLIツールを実行する命令
# supabase login : Supabaseアカウントにログインする
# 実行するとブラウザが開き、ログイン処理が走る
npx supabase login
```

ブラウザが開き、Supabase へのログインが求められます。認証を完了してください。

**ステップ 3: 型定義ファイルの生成**

> **▼ このコードがやること（先に日本語で）:** データベースのテーブル定義を読み取り、それに対応する**TypeScriptの型定義を自動で作って `src/types/supabase.ts` に書き出す**コマンドです。これを実行しておくと、コードで `books` のカラム名を間違えたときにエディタが教えてくれるようになります。`--project-id` の部分は自分のプロジェクトIDに置き換えるのを忘れないでください。

```bash
# プロジェクトのリファレンス ID を確認
# Supabase Dashboard の URL: https://supabase.com/dashboard/project/[ここがリファレンスID]
# または Settings > General > Reference ID で確認

# npx supabase gen types typescript : DBスキーマからTSの型定義を生成するサブコマンド
# --project-id "..."   : 対象のプロジェクトID（自分のものに置き換え）
# --schema public      : publicスキーマのテーブルを対象にする（標準のスキーマ名）
# > src/types/supabase.ts : 標準出力をファイルにリダイレクト（ファイルが上書きされる）
npx supabase gen types typescript --project-id "あなたのプロジェクトID" --schema public > src/types/supabase.ts
```

> **注意:** `あなたのプロジェクトID` は Supabase Dashboard の URL に含まれる英数字の文字列です。

**ステップ 4: 型定義ディレクトリの作成**

事前にディレクトリを作成しておく必要があります:

```bash
# src/types ディレクトリがなければ作る
mkdir -p src/types
```

**ステップ 5: 生成される型定義ファイルの確認**

自動生成された `src/types/supabase.ts` は以下のような内容になります（一部抜粋）:

```typescript
// src/types/supabase.ts
// このファイルは自動生成されます。手動で編集しないでください。

// Json型 : PostgreSQLのjsonb等で扱える値の型を表現
// | はTypeScriptのユニオン型（このいずれかの型をとる）
export type Json =
  | string                                    // 文字列
  | number                                    // 数値
  | boolean                                   // 真偽値
  | null                                      // null
  | { [key: string]: Json | undefined }       // オブジェクト（キーは文字列、値は再帰的にJson）
  | Json[];                                   // Jsonの配列

// Database型 : DB全体のスキーマを表す型
// 「スキーマ → テーブル → Row/Insert/Update」のネスト構造
export type Database = {
  public: {                                   // publicスキーマ
    Tables: {                                 // テーブル一覧
      books: {                                // booksテーブル
        Row: {                                // SELECTで返ってくる「1行」の型
          id: string;                         // uuid → string
          title: string;                      // text NOT NULL → string
          author: string;                     // text NOT NULL → string
          publisher: string | null;           // text NULLable → string | null
          published_date: string | null;     // date → ISO形式の文字列 or null
          rating: number | null;              // integer → number | null
          status: string | null;              // text → string | null
          notes: string | null;
          cover_url: string | null;
          created_at: string | null;          // timestamptz → ISO文字列
          updated_at: string | null;
        };
        Insert: {                             // INSERT時に渡すデータの型
          id?: string;                        // ? は「省略可」。DEFAULTがあるので省略OK
          title: string;                      // NOT NULL なので必須
          author: string;                     // NOT NULL なので必須
          publisher?: string | null;          // NULL許可なので省略可
          published_date?: string | null;
          rating?: number | null;
          status?: string | null;             // DEFAULTがあるので省略可
          notes?: string | null;
          cover_url?: string | null;
          created_at?: string | null;
          updated_at?: string | null;
        };
        Update: {                             // UPDATE時に渡すデータの型
          id?: string;                        // 全カラムが省略可（部分更新を想定）
          title?: string;
          author?: string;
          publisher?: string | null;
          published_date?: string | null;
          rating?: number | null;
          status?: string | null;
          notes?: string | null;
          cover_url?: string | null;
          created_at?: string | null;
          updated_at?: string | null;
        };
      };
    };
    Views: {                                  // ビュー（仮想テーブル）一覧。今は空
      [_ in never]: never;
    };
    Functions: {                              // ストアド関数一覧。今は空
      [_ in never]: never;
    };
    Enums: {                                  // enum型一覧。今は空
      [_ in never]: never;
    };
  };
};
```

**型定義の3つの種類:**

| 型 | 説明 | 使い方 |
|----|------|--------|
| `Row` | SELECT で取得される行の型。全カラムが含まれる | データ表示時に使用 |
| `Insert` | INSERT で送信するデータの型。デフォルト値があるカラムは optional（`?`） | データ作成時に使用 |
| `Update` | UPDATE で送信するデータの型。全カラムが optional | データ更新時に使用 |

**ステップ 6: 便利な型エイリアスの作成**

コードで使いやすいように、型のエイリアス（別名）を作成しておきましょう。

> **▼ このコードがやること（先に日本語で）:** 自動生成された長い型（`Database['public']['Tables']['books']['Row']` など）に、`Book` や `BookInsert` といった**短い別名（エイリアス）を付けてまとめる**ファイルです。こうしておくと、各ファイルで毎回長い型名を書かずに済みます。あわせて、読書状態の3つの値だけを許す型と、その**日本語ラベル**も用意しています。

```typescript
// src/types/book.ts
// books テーブル専用の型エイリアスをまとめたファイル

// 自動生成された Database 型を取り込む
// import type : 型だけ取り込む（実行時のコードには出力されない）
// './supabase' は同じ src/types フォルダの supabase.ts
import type { Database } from './supabase';

// books テーブルの型エイリアス
// Database['public']['Tables']['books']['Row'] という長いパスを Book で再利用可能にする
// ['public'] = publicスキーマ、 ['Tables'] = テーブル群、 ['books'] = booksテーブル
// ['Row'] = SELECTで返ってくる行の型
export type Book = Database['public']['Tables']['books']['Row'];
// INSERT 用の型（省略可能なフィールドあり）
export type BookInsert = Database['public']['Tables']['books']['Insert'];
// UPDATE 用の型（全フィールド省略可能）
export type BookUpdate = Database['public']['Tables']['books']['Update'];

// 読書状態の型
// リテラルユニオン型 : この3つの文字列以外を許さない型
// statusカラムは text だが、CHECK制約のおかげで実質この3つしか入らない
export type BookStatus = 'reading' | 'completed' | 'want_to_read';

// 読書状態の日本語ラベル
// Record<K, V> : すべてのキーKに対して値がVの型 を作るユーティリティ型
// Record<BookStatus, string> = { reading: string, completed: string, want_to_read: string }
export const BOOK_STATUS_LABELS: Record<BookStatus, string> = {
  reading: '読書中',       // 'reading' を画面表示するときの日本語
  completed: '読了',       // 'completed' の表示
  want_to_read: '読みたい', // 'want_to_read' の表示
};
```

**ステップ 7: package.json にスクリプトを追加**

型生成を簡単に実行できるよう、`package.json` にスクリプトを追加します。

```json
{
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "gen:types": "supabase gen types typescript --project-id \"あなたのプロジェクトID\" --schema public > src/types/supabase.ts"
  }
}
```

> **`gen:types` スクリプトの中身解説:**
> - `supabase gen types typescript` : Supabase CLI のサブコマンド。TypeScriptの型を生成
> - `--project-id "..."` : 対象プロジェクトIDを指定
> - `--schema public` : publicスキーマを対象に
> - `> src/types/supabase.ts` : 出力先ファイル（リダイレクト）
> - JSON内のダブルクォートはエスケープ（`\"`）が必要

これにより、テーブル定義を変更した後は以下のコマンドで型を再生成できます:

```bash
# npm run スクリプト名 で package.json の scripts を実行
npm run gen:types
```

---

## 7. Supabase の基本操作（CRUD）

CRUD とは、データベースの4つの基本操作の頭文字をとったものです。

| 操作 | 意味 | SQL | Supabase メソッド |
|------|------|-----|-------------------|
| **C**reate | 作成 | INSERT | `.insert()` |
| **R**ead | 読み取り | SELECT | `.select()` |
| **U**pdate | 更新 | UPDATE | `.update()` |
| **D**elete | 削除 | DELETE | `.delete()` |

以下のデータフローを意識しながら、各操作を学んでいきましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <div style="display: flex; align-items: center; justify-content: center; gap: 10px; flex-wrap: wrap;">
    <!-- Frontend -->
    <div style="background: #eff6ff; border: 2px solid #3b82f6; border-radius: 10px; padding: 14px 18px; text-align: center; min-width: 130px;">
      <div style="font-size: 11px; color: #64748b; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 6px;">Frontend</div>
      <div style="font-size: 18px;">🖥️</div>
      <div style="font-weight: 700; color: #1e40af; font-size: 12px;">React Component</div>
    </div>
    <!-- Arrow right -->
    <div style="text-align: center; min-width: 40px;">
      <div style="color: #3b82f6; font-size: 18px;">→</div>
      <div style="font-size: 9px; color: #64748b; max-width: 60px; line-height: 1.2;">.select() .insert() .update() .delete()</div>
    </div>
    <!-- Supabase Client -->
    <div style="background: #fff7ed; border: 2px solid #f59e0b; border-radius: 10px; padding: 14px 18px; text-align: center; min-width: 140px;">
      <div style="font-size: 11px; color: #64748b; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 6px;">Supabase Client</div>
      <div style="font-size: 18px;">📡</div>
      <div style="font-weight: 700; color: #92400e; font-size: 12px;">supabase.from('books')</div>
    </div>
    <!-- Arrow right -->
    <div style="text-align: center; min-width: 40px;">
      <div style="color: #3ecf8e; font-size: 18px;">→</div>
      <div style="font-size: 9px; color: #64748b; line-height: 1.2;">HTTPS</div>
    </div>
    <!-- Supabase Server -->
    <div style="background: #ecfdf5; border: 2px solid #3ecf8e; border-radius: 10px; padding: 14px 18px; text-align: center; min-width: 130px;">
      <div style="font-size: 11px; color: #64748b; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 6px;">Supabase Server</div>
      <div style="font-size: 18px;">🔌</div>
      <div style="font-weight: 700; color: #065f46; font-size: 12px;">PostgREST API</div>
      <div style="color: #3ecf8e; font-size: 12px; margin: 4px 0;">↓ SQL</div>
      <div style="font-size: 18px;">🐘</div>
      <div style="font-weight: 700; color: #065f46; font-size: 12px;">PostgreSQL</div>
    </div>
  </div>
  <!-- Return flow -->
  <div style="text-align: center; margin-top: 10px;">
    <div style="display: inline-block; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 8px 24px; font-size: 11px; color: #64748b;">
      ← <strong style="color: #065f46;">JSON Response</strong> ← <strong style="color: #92400e;">data / error</strong> ← <strong style="color: #1e40af;">React re-render</strong>
    </div>
  </div>
</div>

### 7.1 SELECT（データの読み取り）

#### 全件取得

> **▼ このコードがやること（先に日本語で）:** booksテーブルの**全データを取り出す**、CRUDの「R（読み取り）」の基本形です。`.from('books').select('*')` で「booksテーブルの全カラムをちょうだい」という意味になります。Supabase は失敗しても例外を投げず、戻り値の `error` に情報を入れる設計なので、**`error` を必ず確認してから `data` を使う**のが鉄則です。

```typescript
// すべての書籍を取得するクエリ
// await       : このAPI呼び出しが終わるまで待つ（時間がかかる処理）
// supabase    : 6.3で作ったクライアントオブジェクト
// .from('books') : booksテーブルを操作対象にする（テーブル名を指定）
// .select('*')   : 全カラムを取得する（'*' はワイルドカード）
// 戻り値は { data, error } というオブジェクト。分割代入で取り出す
const { data, error } = await supabase
  .from('books')
  .select('*');

// エラーチェック : Supabaseは例外を投げない設計。errorで判定するのが基本
if (error) {
  // console.error : エラーログをコンソールに赤字で出力
  // error.message にユーザー向けの説明文が入っている
  console.error('エラー:', error.message);
  return;  // 関数を抜ける（以降を実行しない）
}

// 成功した場合、data に取得結果が配列で入る
console.log('書籍一覧:', data);
// 結果例:
// [
//   {
//     id: '550e8400-e29b-41d4-a716-446655440000',
//     title: 'ノルウェイの森',
//     author: '村上春樹',
//     publisher: '講談社',
//     published_date: '1987-09-04',
//     rating: 5,
//     status: 'completed',
//     notes: '名作。何度も読み返したい。',
//     cover_url: null,
//     created_at: '2024-01-15T10:30:00.000Z',
//     updated_at: '2024-01-15T10:30:00.000Z'
//   },
//   ...
// ]
```

#### ▼ コードを1つずつ分解して解説

この「全件取得」は、これ以降のCRUD操作すべての土台になる形です。塊ごとに分解します。

##### 解説1: メソッドをつないで「どのテーブルから何を取るか」を組み立てる

```ts
const { data, error } = await supabase
  .from('books')
  .select('*');
```

- `supabase.from('books')` で「`books` テーブルを操作対象にする」と宣言し、続く `.select('*')` で「全カラム（`*`）を取り出す」と指定します。
- このように `.` でメソッドをつないでいく書き方をメソッドチェーンと呼びます。条件や並び替えも、このうしろにつなげて足していけます。
- `await` は「この通信が終わって結果が返るまで待つ」という意味です。DB操作は時間がかかるため必要です。
- 戻り値は `{ data, error }` という形のオブジェクトで、分割代入で `data`（取得結果）と `error`（エラー情報）を同時に取り出しています。

> **用語:** **メソッドチェーン** = `a.b().c()` のように、メソッドを点でつないで処理を組み立てる書き方。**分割代入** = オブジェクトから必要なプロパティだけを取り出して変数にする書き方。

---

##### 解説2: error を必ず先にチェックする

```ts
if (error) {
  console.error('エラー:', error.message);
  return;
}
```

- Supabaseは失敗しても例外を投げず、`error` に情報を入れて返す設計です。そのため `data` を使う前に必ず `error` を確認します。
- `if (error)` は「`error` に中身があれば（＝失敗していれば）」という判定です。失敗時は `error.message`（人間向けの説明）をログに出します。
- `return` でそこより先（`data` を使う処理）に進まないようにします。これでエラー時に壊れた `data` を触らずに済みます。

> **用語:** **例外（throw）を投げない設計** = エラーを `try/catch` で捕まえるのではなく、戻り値の `error` で受け取る方式。Supabaseはこちらを採用している。

---

#### 特定のカラムのみ取得

> **▼ このコードがやること（先に日本語で）:** 全カラムではなく、**欲しい列だけを指定して取り出す**例です。`.select('title, author, rating')` のようにカンマ区切りで列名を並べます。必要な列だけ取れば通信量が減って表示も速くなる、というのがポイントです。

```typescript
// タイトルと著者のみ取得（データ転送量を減らせる）
// .select('title, author, rating') : カンマ区切りで取得したいカラムを文字列で指定
//   * で全列を取らず、必要な列だけ指定することで通信が軽くなる
const { data, error } = await supabase
  .from('books')
  .select('title, author, rating');

// 結果例: 指定したカラムだけのオブジェクトが配列で返る
// [
//   { title: 'ノルウェイの森', author: '村上春樹', rating: 5 },
//   { title: '人間失格', author: '太宰治', rating: 4 },
//   ...
// ]
```

#### 条件付き取得（フィルタリング）

> **▼ このコードがやること（先に日本語で）:** 全件ではなく**条件に合う行だけ**を取り出す例です。`.gte('rating', 4)` は「rating が 4 以上（greater than or equal）」という絞り込み条件で、SQL の `WHERE rating >= 4` に相当します。`.select(...)` のうしろにこうした条件メソッドをつなげて使う、という形を覚えましょう。

```typescript
// 評価が4以上の書籍のみ取得
// .gte('rating', 4) : rating >= 4 という条件を追加
//   gte = greater than or equal (以上)
//   メソッドチェーンで好きなだけ条件を追加できる
const { data, error } = await supabase
  .from('books')
  .select('*')
  .gte('rating', 4);  // gte = greater than or equal (以上)

// 結果例:
// [
//   { title: 'ノルウェイの森', rating: 5, ... },
//   { title: '人間失格', rating: 4, ... },
// ]
```

> **▼ このコードがやること（先に日本語で）:** 「**ある列の値がぴったり一致する行**」だけを取り出す例です。`.eq('status', 'reading')` は「status が 'reading' と等しい行だけ」を意味し（eq = equal）、SQL の `WHERE status = 'reading'` と同じです。完全一致で絞りたいときの定番メソッドです。

```typescript
// 読書中の書籍のみ取得
// .eq('status', 'reading') : status カラムが 'reading' と等しい行のみ
//   eq = equal （等しい）
//   SQLの WHERE status = 'reading' に相当
const { data, error } = await supabase
  .from('books')
  .select('*')
  .eq('status', 'reading');  // eq = equal (等しい)
```

> **▼ このコードがやること（先に日本語で）:** 完全一致ではなく「**一部に含むかどうか（あいまい検索）**」で絞り込む例です。`.ilike('author', '%村上%')` の `%` は「任意の文字列」を表すワイルドカードで、「authorのどこかに『村上』を含む行」を探します。`ilike` は大文字・小文字を区別しない検索です。

```typescript
// タイトルに「村上」を含む書籍を検索
// .ilike('author', '%村上%')
//   ilike = case-insensitive LIKE。大文字小文字を区別しないパターン一致
//   % はワイルドカード（任意の文字列）
//   '%村上%' は「文字列のどこかに『村上』を含む」を意味する
const { data, error } = await supabase
  .from('books')
  .select('*')
  .ilike('author', '%村上%');  // ilike = 大文字小文字を区別しない部分一致
```

**主なフィルタメソッド一覧:**

| メソッド | SQL 相当 | 説明 | 例 |
|---------|---------|------|-----|
| `.eq(column, value)` | `= value` | 等しい | `.eq('status', 'reading')` |
| `.neq(column, value)` | `!= value` | 等しくない | `.neq('status', 'completed')` |
| `.gt(column, value)` | `> value` | より大きい | `.gt('rating', 3)` |
| `.gte(column, value)` | `>= value` | 以上 | `.gte('rating', 4)` |
| `.lt(column, value)` | `< value` | より小さい | `.lt('rating', 3)` |
| `.lte(column, value)` | `<= value` | 以下 | `.lte('rating', 2)` |
| `.like(column, pattern)` | `LIKE pattern` | パターン一致 | `.like('title', '%森%')` |
| `.ilike(column, pattern)` | `ILIKE pattern` | パターン一致（大文字小文字無視） | `.ilike('author', '%murakami%')` |
| `.is(column, value)` | `IS value` | NULL チェック | `.is('notes', null)` |
| `.in(column, values)` | `IN (values)` | 複数値のいずれか | `.in('status', ['reading', 'completed'])` |

#### ソート

> **▼ このコードがやること（先に日本語で）:** 取り出したデータを**指定した列の順番に並べ替える**例です。`.order('rating', { ascending: false })` は「rating の降順（大きい→小さい）に並べる」という意味で、`ascending: false` が「降順」のスイッチです。`true` にすれば昇順（小さい→大きい）になります。

```typescript
// 評価が高い順にソート
// .order('rating', { ascending: false })
//   第1引数 'rating' : 並び替えのキーとなるカラム
//   第2引数 { ascending: false } : ascending=昇順かどうか。falseで降順（大→小）
//   trueなら昇順（小→大）。デフォルトは true（昇順）
const { data, error } = await supabase
  .from('books')
  .select('*')
  .order('rating', { ascending: false });  // ascending: false = 降順

// 結果例:
// [
//   { title: 'ノルウェイの森', rating: 5, ... },
//   { title: '人間失格', rating: 4, ... },
//   { title: 'こころ', rating: 3, ... },
// ]
```

```typescript
// 作成日時の新しい順にソート
// created_at は ISO文字列だが、文字列としても日時としても降順で同じ並びになる
const { data, error } = await supabase
  .from('books')
  .select('*')
  .order('created_at', { ascending: false });
```

#### ページネーション

> **▼ このコードがやること（先に日本語で）:** データが多いときに**「○件ずつ、○ページ目だけ」取り出す**ページ送りの例です。`.range(開始, 終了)` で取得する行の範囲を指定し、`{ count: 'exact' }` で全体の総件数も一緒にもらいます。総件数を1ページ件数で割って切り上げれば**総ページ数**が分かる、という流れがポイントです。

```typescript
// 1ページあたり10件、1ページ目を取得
// pageSize : 1ページに表示する件数
const pageSize = 10;
// page : 何ページ目を取りたいか（1始まり）
const page = 1;

// .select('*', { count: 'exact' })
//   第2引数 { count: 'exact' } で「該当行の総件数」も一緒に取得する
//   'exact'  : 正確に数える（重いが正確）
//   'planned': おおよそ（軽い）
//   'estimated': 推定値
const { data, error, count } = await supabase
  .from('books')
  .select('*', { count: 'exact' })  // count: 'exact' で総件数も取得
  .order('created_at', { ascending: false })
  // .range(開始インデックス, 終了インデックス) : 行範囲を指定（0始まり、両端含む）
  //   1ページ目(page=1)なら 0〜9、 2ページ目(page=2)なら 10〜19 の行を取る
  .range((page - 1) * pageSize, page * pageSize - 1);  // range(開始, 終了)

console.log('データ:', data);       // 10件のデータ
console.log('総件数:', count);      // テーブル内の全件数
// Math.ceil(x) : 切り上げ。総件数÷1ページ件数 を切り上げると総ページ数になる
// count ?? 0  : count が null/undefined なら 0 を使う（Nullish Coalescing演算子）
console.log('総ページ数:', Math.ceil((count ?? 0) / pageSize));
```

#### 1件だけ取得

> **▼ このコードがやること（先に日本語で）:** 配列ではなく、**ちょうど1件のデータをオブジェクトとして取り出す**例です。`.eq('id', ...)` で対象を1件に絞り、`.single()` を付けると「配列の中の1個」ではなく「オブジェクトそのもの」が返ります。詳細ページなど「1件だけ表示したい」ときに便利ですが、結果が0件や2件以上だとエラーになる点に注意です。

```typescript
// ID を指定して1件取得
// .eq('id', '...') で対象を1件に絞り込み
// .single() で「配列ではなく1個のオブジェクト」として受け取る
//   結果が0件 or 2件以上だとエラーになる
const { data, error } = await supabase
  .from('books')
  .select('*')
  .eq('id', '550e8400-e29b-41d4-a716-446655440000')
  .single();  // single() で1件のみ取得（配列ではなくオブジェクトが返る）

// 結果例: 配列ではなくオブジェクトが返る
// {
//   id: '550e8400-e29b-41d4-a716-446655440000',
//   title: 'ノルウェイの森',
//   ...
// }
```

> **`.single()` の注意点:**
> `.single()` は結果が0件または2件以上の場合にエラーを返します。必ず1件だけ返ることが保証される場面（主キーで検索する場合など）でのみ使用してください。0件の可能性がある場合は `.maybeSingle()` を使用します（0件なら data が null になり、エラーにはなりません）。

### 7.2 INSERT（データの作成）

#### 単一行の挿入

> **▼ このコードがやること（先に日本語で）:** 新しい書籍を**1件追加する**、CRUDの「C（作成）」の基本形です。追加したい値をオブジェクトにまとめ、`.insert(...)` に渡します。`id` や `created_at` のように**DBが自動で埋める列は省略してOK**です。末尾の `.select()` を付けると、追加された行（自動生成されたidを含む）が返ってくるのがポイントです。

```typescript
// BookInsert型をインポート（INSERT用のオブジェクト型）
// DEFAULTがあるidやcreated_atは省略可、NOT NULLのtitle/authorは必須
import type { BookInsert } from '@/types/book';

// 新しい書籍を1件追加
// 型注釈 : BookInsert を付けることでミスタイプ・必須項目漏れを検出できる
const newBook: BookInsert = {
  title: 'ノルウェイの森',              // 必須
  author: '村上春樹',                   // 必須
  publisher: '講談社',                  // 任意
  published_date: '1987-09-04',         // 任意。'YYYY-MM-DD' 形式の文字列
  rating: 5,                            // 任意（CHECK制約で 1〜5 のみ）
  status: 'completed',                  // 任意。指定しないと 'want_to_read'
  notes: '名作。何度も読み返したい。',   // 任意
  // id, cover_url, created_at, updated_at は省略 → DBが自動で埋める
};

// .insert(newBook) : booksテーブルに newBook を1件追加するクエリ
// .select()        : 挿入された結果を返してもらう（idやcreated_atも入る）
const { data, error } = await supabase
  .from('books')
  .insert(newBook)
  .select();  // .select() を付けると、挿入したデータが返る

if (error) {
  console.error('挿入エラー:', error.message);
  return;
}

console.log('挿入されたデータ:', data);
// 結果例:
// [
//   {
//     id: '自動生成されたUUID',
//     title: 'ノルウェイの森',
//     author: '村上春樹',
//     publisher: '講談社',
//     published_date: '1987-09-04',
//     rating: 5,
//     status: 'completed',
//     notes: '名作。何度も読み返したい。',
//     cover_url: null,
//     created_at: '2024-01-15T10:30:00.000Z',
//     updated_at: '2024-01-15T10:30:00.000Z'
//   }
// ]
```

#### ▼ コードを1つずつ分解して解説

INSERT（作成）は「①追加する値を用意 → ②`.insert()` で送る → ③`.select()` で結果を受け取る」という流れです。

##### 解説1: 追加するデータをオブジェクトにまとめる

```ts
const newBook: BookInsert = {
  title: 'ノルウェイの森',              // 必須
  author: '村上春樹',                   // 必須
  publisher: '講談社',                  // 任意
  // ...
  // id, cover_url, created_at, updated_at は省略 → DBが自動で埋める
};
```

- 追加したい値を1つのオブジェクト（`{ }`）にまとめます。キーが列名、値が入れる中身です。
- `: BookInsert` という型注釈を付けると、必須項目（`title`・`author`）の入れ忘れや、存在しない列名のタイプミスをエディタが教えてくれます。
- `id`・`created_at` のように **DEFAULT が設定された列は省略できます**。省略するとDB側が自動で値を埋めます。

> **用語:** **型注釈** = 変数の中身の「型（形）」を明示すること。`BookInsert` は「INSERTで渡してよいデータの形」を表す型。

---

##### 解説2: .insert() で追加し、.select() で結果を受け取る

```ts
const { data, error } = await supabase
  .from('books')
  .insert(newBook)
  .select();  // .select() を付けると、挿入したデータが返る
```

- `.insert(newBook)` で「`newBook` を1件追加する」クエリになります。
- 末尾の `.select()` がポイントです。これを**付けないと**、追加は成功しても戻り値の `data` は空になります（性能上の理由）。
- `.select()` を付けると、自動生成された `id` や `created_at` を含む「実際に保存された行」が `data` に返ってきます。

> **用語:** **insert** = テーブルに新しい行を追加する操作（SQLの INSERT に相当）。追加後の行を使いたいときは `.select()` をつなげる。

---

> **`.select()` を付ける理由:**
> `.insert()` だけだとレスポンスにデータが含まれません（パフォーマンス上の理由）。挿入したデータ（自動生成された id や created_at を含む）を取得したい場合は `.select()` を付けてください。

#### 複数行の挿入

> **▼ このコードがやること（先に日本語で）:** 1件ずつではなく、**複数の書籍をまとめて一度に追加する**例です。`.insert(...)` に**オブジェクトの配列**を渡すだけで、何件でも同時に追加できます。1件ずつループするより通信回数が減って速い、というのが利点です。

```typescript
// 複数の書籍を一度に追加
// BookInsert[] : BookInsert の配列型
// 1回の通信で複数行を入れられる（個別にループするより速い）
const newBooks: BookInsert[] = [
  {
    title: '人間失格',                  // 1件目
    author: '太宰治',
    publisher: '新潮社',
    published_date: '1948-06-01',
    rating: 4,
    status: 'completed',
  },
  {
    title: 'こころ',                    // 2件目
    author: '夏目漱石',
    publisher: '岩波書店',
    published_date: '1914-09-01',
    rating: 5,
    status: 'reading',
  },
  {
    title: '銀河鉄道の夜',              // 3件目（最小限の項目のみ）
    author: '宮沢賢治',
    status: 'want_to_read',  // 最低限 title と author があれば OK
  },
];

// .insert に配列を渡すと「複数行同時INSERT」になる
const { data, error } = await supabase
  .from('books')
  .insert(newBooks)
  .select();

if (error) {
  console.error('挿入エラー:', error.message);
  return;
}

// data.length : 返ってきた配列の要素数（=実際に挿入できた件数）
console.log(`${data.length}件の書籍を追加しました`);
```

### 7.3 UPDATE（データの更新）

> **▼ このコードがやること（先に日本語で）:** 既存の書籍の情報を**書き換える**、CRUDの「U（更新）」の基本形です。`.update(...)` に**変えたいフィールドだけ**を渡し、`.eq('id', bookId)` で「どの行を更新するか」を指定します。**この `.eq(...)` を付け忘れると全行が書き換わってしまう**ため、対象の絞り込みが最重要ポイントです。

```typescript
// BookUpdate型 : UPDATE用の型。全フィールドが optional（一部だけ更新できる）
import type { BookUpdate } from '@/types/book';

// 特定の書籍の情報を更新
// bookId : 更新対象の書籍ID（事前に取得しておく）
const bookId = '550e8400-e29b-41d4-a716-446655440000';

// updates : 書き換えたいフィールドだけを含むオブジェクト
const updates: BookUpdate = {
  rating: 4,                       // 評価を4に
  status: 'completed',             // 状態を 'completed' に
  notes: '読了。面白かった！',      // メモを上書き
  // title や author は省略 → 既存値のまま
};

// .update(updates) : updates の内容で行を書き換える
// .eq('id', bookId) : id が bookId の行だけ対象（必須！）
// .select() : 更新後のデータを取得
const { data, error } = await supabase
  .from('books')
  .update(updates)
  .eq('id', bookId)  // 必ず条件を指定すること！
  .select();

if (error) {
  console.error('更新エラー:', error.message);
  return;
}

console.log('更新されたデータ:', data);
// 結果例:
// [
//   {
//     id: '550e8400-e29b-41d4-a716-446655440000',
//     title: 'ノルウェイの森',
//     rating: 4,           ← 更新された
//     status: 'completed', ← 更新された
//     notes: '読了。面白かった！', ← 更新された
//     updated_at: '2024-01-16T15:00:00.000Z', ← トリガーにより自動更新
//     ...
//   }
// ]
```

#### ▼ コードを1つずつ分解して解説

UPDATE（更新）は「①変えたい値を用意 → ②`.update()` で送り → ③`.eq()` で対象を絞る」という流れです。**対象の絞り込みが命綱**です。

##### 解説1: 変えたいフィールドだけをオブジェクトにする

```ts
const updates: BookUpdate = {
  rating: 4,                       // 評価を4に
  status: 'completed',             // 状態を 'completed' に
  notes: '読了。面白かった！',      // メモを上書き
  // title や author は省略 → 既存値のまま
};
```

- 更新では「変えたい列だけ」を書きます。書かなかった列（`title` など）は既存の値のまま残ります。
- `: BookUpdate` 型は「すべての列が省略可（optional）」になっており、部分的な更新を安全に書けます。

> **用語:** **部分更新** = 行のすべての列ではなく、一部の列だけを書き換えること。Supabaseの `.update()` は渡した列だけを更新する。

---

##### 解説2: .eq() で「どの行を更新するか」を必ず絞る

```ts
const { data, error } = await supabase
  .from('books')
  .update(updates)
  .eq('id', bookId)  // 必ず条件を指定すること！
  .select();
```

- `.update(updates)` で「`updates` の内容で書き換える」、`.eq('id', bookId)` で「`id` が `bookId` の行だけを対象にする」と指定します（eq = equal）。
- **この `.eq(...)` を付け忘れると、テーブルの全行が同じ値に書き換わります。** これは非常に危険なので、更新時は必ず対象を絞ります。
- `.select()` を付けると、更新後の行（トリガーで自動更新された `updated_at` を含む）が返ってきます。

> **用語:** **.eq(列, 値)** = 「その列が指定した値と等しい行だけ」に絞り込むメソッド。SQLの `WHERE 列 = 値` に相当する。

---

> **重要: `.eq()` 等の条件を必ず指定すること！**
>
> `.update()` に条件を付けないと **テーブルの全行が更新されてしまいます**。これは非常に危険です。必ず `.eq('id', bookId)` のような条件を付けてください。

#### 読書状態だけを更新する例

```typescript
// ステータスだけを変更（オブジェクトには1フィールドだけ書く）
const { data, error } = await supabase
  .from('books')
  .update({ status: 'completed' })  // 直接オブジェクトリテラルを渡してもOK
  .eq('id', bookId)                  // 対象の絞り込み（忘れずに!）
  .select();
```

### 7.4 DELETE（データの削除）

> **▼ このコードがやること（先に日本語で）:** 指定した書籍を**削除する**、CRUDの「D（削除）」の基本形です。`.delete()` のあとに `.eq('id', bookId)` を付けて「どの行を消すか」を指定します。**条件を付け忘れると全行が消え、しかも復元できない**ため、削除の絞り込みは何より慎重に。ここでは結果データは使わないので `error` だけを受け取っています。

```typescript
// 特定の書籍を削除
const bookId = '550e8400-e29b-41d4-a716-446655440000';

// .delete() : 削除クエリの宣言。括弧内に引数なし
// .eq('id', bookId) : 対象を1件に絞り込む（必須！）
// 戻り値からは data を分割代入していない → 結果は不要、errorだけ気にする
const { error } = await supabase
  .from('books')
  .delete()
  .eq('id', bookId);  // 必ず条件を指定すること！

if (error) {
  console.error('削除エラー:', error.message);
  return;
}

console.log('書籍を削除しました');
```

#### ▼ コードを1つずつ分解して解説

DELETE（削除）は最も慎重に扱う操作です。塊ごとに見ていきます。

##### 解説1: .delete() に .eq() で対象を絞る

```ts
const { error } = await supabase
  .from('books')
  .delete()
  .eq('id', bookId);  // 必ず条件を指定すること！
```

- `.delete()` で「削除する」クエリを宣言し、`.eq('id', bookId)` で「`id` が `bookId` の行だけ」に対象を絞ります。
- **`.eq(...)` を付け忘れると全行が削除され、しかも復元できません。** 削除では対象の絞り込みが何より重要です。
- ここでは戻り値から `data` を取り出していません。削除した中身は不要で、成功・失敗（`error`）だけ確認すればよいからです。

> **用語:** **delete** = テーブルから行を削除する操作（SQLの DELETE に相当）。条件なしの delete は全件削除になるため、必ず絞り込む。

---

##### 解説2: 失敗していないかを error で確認する

```ts
if (error) {
  console.error('削除エラー:', error.message);
  return;
}
```

- 他の操作と同じく、Supabaseは削除の失敗も `error` で返します。`if (error)` で失敗を検知し、メッセージを出して `return` で処理を止めます。
- 削除されたデータを確認したい場合は、このあとの例のように `.eq(...).select()` と `.select()` をつなげると、消した行の内容が `data` に返ってきます。

> **用語:** **error.message** = エラーの内容を人間向けに説明した文字列。ログに出すと原因を追いやすい。

---

> **重要: `.eq()` 等の条件を必ず指定すること！**
>
> `.delete()` に条件を付けないと **テーブルの全行が削除されてしまいます**。復元はできません。必ず条件を付けてください。

#### 削除されたデータを確認する

```typescript
// 削除されたデータを返り値で確認したい場合
// .select() を付けると、削除された行の内容が返ってくる
const { data, error } = await supabase
  .from('books')
  .delete()
  .eq('id', bookId)
  .select();  // select() を付けると削除されたデータが返る

console.log('削除されたデータ:', data);
```

### 7.5 エラーハンドリングのパターン

Supabase の操作では、常にエラーが発生する可能性があります。以下は推奨されるエラーハンドリングのパターンです。

> **▼ このコードがやること（先に日本語で）:** DB操作で**エラーが起きたときの、おすすめの対処の型（パターン）**を関数にまとめた例です。`error` が入っていたら中身（message・code・details・hint）をログに出し、呼び出し元が気づけるよう改めてエラーを投げ直します。問題が起きたときに「何が原因か」を追いやすくするための、実戦的な書き方だと捉えてください。

```typescript
// 汎用的なエラーハンドリング関数
// async : この関数はPromiseを返す非同期関数
async function fetchBooks() {
  // SELECTクエリの実行
  const { data, error } = await supabase
    .from('books')
    .select('*')
    .order('created_at', { ascending: false });

  // エラーが存在する場合のハンドリング
  if (error) {
    // エラーオブジェクトの中身をオブジェクトで整理して出力
    console.error('Supabase エラー:', {
      message: error.message,  // エラーメッセージ（人間向け説明）
      code: error.code,        // エラーコード（PostgrestErrorのコード文字列）
      details: error.details,  // 詳細情報（DBが返したdetails）
      hint: error.hint,        // ヒント（修正方法の提案。PostgreSQLが付けてくれる）
    });
    // throw new Error(...) : 上位関数に再スローしてUI側でcatchできるようにする
    // テンプレートリテラル `${...}` で文字列の中に変数を埋め込める
    throw new Error(`書籍の取得に失敗しました: ${error.message}`);
  }

  // 成功時はデータを返す
  return data;
}
```

---

## 8. テストデータの投入

### 8.1 SQL でサンプルデータを投入

Supabase の SQL Editor で以下の SQL を実行して、テスト用のサンプルデータを5件投入します。

> **▼ このコードがやること（先に日本語で）:** 動作確認用に、**書籍データを5件まとめてテーブルに追加する**SQLです。`INSERT INTO books (...) VALUES (...), (...), ...` のように、`VALUES` のあとに括弧をカンマで並べると一度に複数行を入れられます。各行の値は、先頭で並べたカラム名と**同じ順番**で書くのがルールです。

```sql
-- ============================================
-- テストデータの投入
-- 書籍管理アプリのサンプルデータ
-- ============================================

-- INSERT INTO books : books テーブルに行を追加
-- (title, author, publisher, published_date, rating, status, notes) : 値を入れるカラム名
-- VALUES (...), (...), ... : 複数行を一度に追加する書き方（カンマで区切る）
INSERT INTO books (title, author, publisher, published_date, rating, status, notes) VALUES
-- 1件目: ノルウェイの森
(
  'ノルウェイの森',     -- title
  '村上春樹',           -- author
  '講談社',             -- publisher
  '1987-09-04',         -- published_date（YYYY-MM-DD形式の日付リテラル）
  5,                    -- rating（1〜5の整数）
  'completed',          -- status（CHECK制約で許可された値）
  '名作。静かで美しい文体に引き込まれた。何度も読み返したくなる作品。'  -- notes
),
-- 2件目: 人間失格
(
  '人間失格',
  '太宰治',
  '新潮社',
  '1948-06-25',
  4,
  'completed',
  '太宰治の代表作。人間の弱さと苦悩が痛いほど伝わってくる。'
),
-- 3件目: こころ（読書中）
(
  'こころ',
  '夏目漱石',
  '岩波書店',
  '1914-09-20',
  5,
  'reading',
  '先生の手紙の部分を読んでいる。明治時代の人間関係の複雑さが興味深い。'
),
-- 4件目: 銀河鉄道の夜（未読、評価NULL）
(
  '銀河鉄道の夜',
  '宮沢賢治',
  '岩波書店',
  '1934-01-01',
  NULL,                 -- rating は NULL（未評価）
  'want_to_read',       -- まだ読みたいリストの段階
  '友人に勧められた。幻想的な世界観が気になる。'
),
-- 5件目: コンビニ人間
(
  'コンビニ人間',
  '村田沙耶香',
  '文藝春秋',
  '2016-07-27',
  4,
  'completed',
  '芥川賞受賞作。「普通」とは何かを考えさせられた。読みやすく一気に読了。'
);
-- 末尾のセミコロンで1つのINSERT文の終わり
```

実行後、`Success. 5 rows affected.` と表示されれば成功です。

### 8.2 Supabase JavaScript クライアントからデータを投入する方法

SQL の代わりに、JavaScript/TypeScript コードからもテストデータを投入できます。

> **▼ このコードがやること（先に日本語で）:** 先ほどのSQLと同じ「サンプルデータ5件の投入」を、**TypeScriptのコードで行う**スクリプトです。`seedBooks` 関数の中で、まず既存データを全部消してから（テスト環境を毎回まっさらにするため）、配列にまとめたデータを `.insert(...)` で一括追加しています。`async`/`await` で「DB処理の完了を待ってから次へ進む」流れになっている点に注目してください。

```typescript
// テストデータの投入スクリプト（開発時のみ使用）

// 6.3で作ったSupabaseクライアントを取り込む
import { supabase } from '@/lib/supabase';
// INSERT用の型を取り込む
import type { BookInsert } from '@/types/book';

// 投入したいデータの配列
const sampleBooks: BookInsert[] = [
  {
    title: 'ノルウェイの森',
    author: '村上春樹',
    publisher: '講談社',
    published_date: '1987-09-04',
    rating: 5,
    status: 'completed',
    notes: '名作。静かで美しい文体に引き込まれた。何度も読み返したくなる作品。',
  },
  {
    title: '人間失格',
    author: '太宰治',
    publisher: '新潮社',
    published_date: '1948-06-25',
    rating: 4,
    status: 'completed',
    notes: '太宰治の代表作。人間の弱さと苦悩が痛いほど伝わってくる。',
  },
  {
    title: 'こころ',
    author: '夏目漱石',
    publisher: '岩波書店',
    published_date: '1914-09-20',
    rating: 5,
    status: 'reading',
    notes: '先生の手紙の部分を読んでいる。明治時代の人間関係の複雑さが興味深い。',
  },
  {
    title: '銀河鉄道の夜',
    author: '宮沢賢治',
    publisher: '岩波書店',
    published_date: '1934-01-01',
    status: 'want_to_read',  // ratingとnotesは省略
    notes: '友人に勧められた。幻想的な世界観が気になる。',
  },
  {
    title: 'コンビニ人間',
    author: '村田沙耶香',
    publisher: '文藝春秋',
    published_date: '2016-07-27',
    rating: 4,
    status: 'completed',
    notes: '芥川賞受賞作。「普通」とは何かを考えさせられた。読みやすく一気に読了。',
  },
];

// async関数 : 非同期処理をまとめて書ける関数
async function seedBooks() {
  // 既存データを削除（テスト環境のみ）
  // .delete() に .neq() を付けて「全件削除」を間接的に表現
  //   Supabaseは「条件なしのdelete()」を安全のため拒否することがあるので
  //   .neq('id', 存在しないID) で実質全件選択する裏ワザ
  // 戻り値オブジェクトから error だけを取り出し、別名 deleteError に
  const { error: deleteError } = await supabase
    .from('books')
    .delete()
    .neq('id', '00000000-0000-0000-0000-000000000000'); // 全行削除の安全策

  if (deleteError) {
    console.error('削除エラー:', deleteError.message);
    return;
  }

  // サンプルデータを挿入
  // .insert(sampleBooks) で配列を一括INSERT
  // .select() で挿入されたデータ（idなど）を取得
  const { data, error } = await supabase
    .from('books')
    .insert(sampleBooks)
    .select();

  if (error) {
    console.error('挿入エラー:', error.message);
    return;
  }

  // テンプレートリテラルで件数を表示
  console.log(`${data.length}件のサンプルデータを投入しました`);
  // forEach : 配列の各要素について順番にコールバックを実行
  // (book) => { ... } : アロー関数。各要素を book として受け取る
  data.forEach((book) => {
    console.log(`  - ${book.title} (${book.author})`);
  });
}

// 実行 : 上で定義した関数を呼び出す
// async関数を呼ぶとPromiseが返るが、ここでは await せずfire-and-forget
seedBooks();
```

### 8.3 Supabase Dashboard での確認方法

テストデータが正しく投入されたかを Supabase Dashboard で確認しましょう。

**ステップ 1: Table Editor を開く**

左メニューの「Table Editor」をクリックします。

**ステップ 2: books テーブルを選択**

テーブル一覧から `books` をクリックします。

**ステップ 3: データの確認**

以下のような画面が表示されるはずです:

```
┌────────────────────────────────────────────────────────────────────┐
│  Table Editor > books                                              │
│                                                                    │
│  [+ Insert row]  [Filter]  [Sort]                                  │
│                                                                    │
│  ┌──────┬────────────────┬──────────┬────────┬────────┬─────────┐  │
│  │ id   │ title          │ author   │ rating │ status │ notes   │  │
│  ├──────┼────────────────┼──────────┼────────┼────────┼─────────┤  │
│  │ 3f.. │ ノルウェイの森 │ 村上春樹 │ 5      │ compl..│ 名作... │  │
│  │ 7a.. │ 人間失格       │ 太宰治   │ 4      │ compl..│ 太宰... │  │
│  │ b2.. │ こころ         │ 夏目漱石 │ 5      │ readi..│ 先生... │  │
│  │ e5.. │ 銀河鉄道の夜   │ 宮沢賢治 │ NULL   │ want_..│ 友人... │  │
│  │ 1c.. │ コンビニ人間   │ 村田沙耶香│ 4     │ compl..│ 芥川... │  │
│  └──────┴────────────────┴──────────┴────────┴────────┴─────────┘  │
│                                                                    │
│  Showing 5 rows                                                    │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

**確認ポイント:**

1. 5件のデータがすべて表示されていること
2. `id` が UUID 形式で自動生成されていること
3. `rating` の値が 1〜5 の範囲内であること（銀河鉄道の夜は NULL）
4. `status` の値が `reading` / `completed` / `want_to_read` のいずれかであること
5. `created_at` と `updated_at` が自動的に設定されていること

**ステップ 4: SQL Editor でデータを確認（オプション）**

SQL Editor で以下のクエリを実行して、データを確認することもできます:

> **▼ このコードがやること（先に日本語で）:** 投入したデータを**いろいろな角度から確認する**SELECT文の例です。`ORDER BY` で並べ替え、`WHERE` で絞り込み、`COUNT(*)` で件数を数え、`GROUP BY` で「statusごとの件数」のように**グループに分けて集計**します。集計（COUNTやGROUP BY）はデータの全体像をつかむのに便利な道具だと覚えておきましょう。

```sql
-- 全件取得
-- SELECT * : 全カラム取得
-- FROM books : booksテーブルから
-- ORDER BY created_at DESC : created_at の降順（新しい順）に並べる
SELECT * FROM books ORDER BY created_at DESC;

-- 読了済みの書籍のみ取得
-- SELECT title, author, rating : 必要なカラムだけ取得
-- WHERE status = 'completed' : status が 'completed' の行のみ
-- ORDER BY rating DESC : ratingの降順（高い順）
SELECT title, author, rating FROM books WHERE status = 'completed' ORDER BY rating DESC;

-- 件数の確認
-- COUNT(*) : 行数を数える集約関数。テーブル全体の行数を返す
SELECT COUNT(*) FROM books;

-- ステータスごとの件数
-- GROUP BY status : 同じstatusの行をグループ化
-- COUNT(*) as count : 各グループの件数を取得し、列名を count にする
SELECT status, COUNT(*) as count FROM books GROUP BY status;
```

---

## 発展: アプリでは使っていない重要なSupabase機能

ここまでで、books テーブルを使った基本的なデータの読み書き（CRUD）と、RLS の基礎は身につきました。この章の本編で作るアプリでは使いませんが、実際のサービスを作るときにほぼ必ず必要になる、Supabase の重要な機能を「最小サンプル」で紹介します。

> **この発展セクションの読み方:** どれも独立した小さな例なので、興味のあるものから読んで構いません。各トピックの最初に「いつ使うか・なぜ必要か」を必ず書いているので、まずはそこだけ読んで「自分のアプリに必要そうか」を判断するのがおすすめです。コードは「今すぐ書く」ものではなく「将来必要になったときに戻ってくる辞書」として使ってください。

### 認証（Auth）: サインアップ・ログイン・ログアウト・現在のユーザー取得

> **▼ このコードがやること（先に日本語で）:** ユーザーが**メールアドレスとパスワードで会員登録（サインアップ）し、ログイン・ログアウトする**ための一式です。「誰がアクセスしているか」をアプリが知るための仕組みで、「自分の投稿だけ表示する」「ログインしていない人には見せない」といった機能の土台になります。Supabase がパスワードの暗号化やログイン状態の管理を全部やってくれるので、私たちは用意された関数を呼ぶだけです。

```ts
// ① メール＋パスワードで新規登録（サインアップ）
// supabase.auth      : 認証まわりの機能がまとまったオブジェクト
// .signUp(...)       : 新しいユーザーを作成する関数
// email / password   : 登録するメールアドレスとパスワード
const { data: signUpData, error: signUpError } = await supabase.auth.signUp({
  email: 'taro@example.com',
  password: 'super-secret-password',
});
if (signUpError) {
  console.error('登録に失敗:', signUpError.message);
}

// ② 登録済みのユーザーがログインする
// .signInWithPassword(...) : メールとパスワードでログインする関数
const { data: loginData, error: loginError } = await supabase.auth.signInWithPassword({
  email: 'taro@example.com',
  password: 'super-secret-password',
});
if (loginError) {
  console.error('ログインに失敗:', loginError.message);
}

// ③ 今ログインしているユーザーの情報を取得する
// .getUser() : 現在のログインユーザーを取得する関数（未ログインなら user は null）
const { data: { user } } = await supabase.auth.getUser();
console.log('今ログイン中のユーザー:', user?.email); // 例: taro@example.com

// ④ ログアウトする
// .signOut() : ログイン状態を解除する関数
const { error: signOutError } = await supabase.auth.signOut();
if (signOutError) {
  console.error('ログアウトに失敗:', signOutError.message);
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: サインアップ（新規会員登録）

```ts
const { data: signUpData, error: signUpError } = await supabase.auth.signUp({
  email: 'taro@example.com',
  password: 'super-secret-password',
});
```

- `supabase.auth` は、ログインまわりの機能がまとまった入れ口です。`.from('books')` がテーブル操作の入り口だったのに対し、`.auth` は「人（ユーザー）」を扱う入り口だと考えてください。
- `.signUp({ email, password })` を呼ぶと、新しいユーザーがデータベースに登録されます。パスワードはそのまま保存されるのではなく、Supabase が安全な形（暗号化したもの）に変換して保存してくれます。
- これまでのCRUDと同じで、戻り値は `{ data, error }` の形です。`data: signUpData` のように書いているのは、後のログインの `data` と名前がぶつからないように**別名をつけている**だけです。

> **用語:** **サインアップ** = アプリに初めて登録すること（会員登録）。**ログイン** = すでに登録済みの人が「これは私です」と認証して入ること。

---

##### 解説2: ログイン（既存ユーザーの認証）

```ts
const { data: loginData, error: loginError } = await supabase.auth.signInWithPassword({
  email: 'taro@example.com',
  password: 'super-secret-password',
});
```

- `.signInWithPassword(...)` は「メールとパスワードが正しいか確認して、正しければログインさせる」関数です。
- ログインに成功すると、Supabase はブラウザの中に「ログイン証明書」のようなもの（トークン）を保存します。これ以降の `supabase.from('books')...` などの操作は、自動的に「このユーザーとして」実行されるようになります。
- 以降に出てくる RLS ポリシーの `auth.uid()`（自分のID）は、このログイン情報をもとに決まります。

> **用語:** **トークン** = 「ログイン済みであること」を証明する文字列。毎回パスワードを送らなくても、このトークンを見せれば本人だと分かる仕組み。

---

##### 解説3: 今ログイン中のユーザーを取得する

```ts
const { data: { user } } = await supabase.auth.getUser();
console.log('今ログイン中のユーザー:', user?.email);
```

- `.getUser()` で「今このブラウザで誰がログインしているか」を取り出せます。ログインしていなければ `user` は `null`（誰もいない）になります。
- `user?.email` の `?.`（オプショナルチェーン）は、「`user` が `null` でなければ `.email` を取り出す。`null` なら何もせず `undefined` を返す」という安全な書き方です。`user` が `null` のときに `user.email` と書くとエラーになるのを防ぎます。
- 「ログインしていなければログイン画面に飛ばす」といった分岐は、この `user` が `null` かどうかで判断します。

---

##### 解説4: ログアウト

```ts
const { error: signOutError } = await supabase.auth.signOut();
```

- `.signOut()` を呼ぶと、ブラウザに保存されていたログイン証明書（トークン）が消され、未ログイン状態に戻ります。
- ログアウト後は `getUser()` の `user` が `null` になり、自分専用のデータも見えなくなります（次のRLSの話につながります）。

### 認証後の厳密なRLSポリシー: 「自分のデータだけ」を守る

> **▼ このコードがやること（先に日本語で）:** ログイン機能とセットで使う、**「自分が作ったデータだけ読める・書ける」**という厳しいルール（ポリシー）を SQL で定義します。本編のRLSは「ログインしていれば誰でもOK」レベルでしたが、実際のサービスでは「他人の家計簿や日記が見えてしまう」と大問題です。ここでは、テーブルに `user_id`（持ち主のID）列を足し、その列が**今ログインしている人のIDと一致する行だけ**操作を許可するようにします。

```sql
-- ============================================================================
-- 前提: books テーブルに「持ち主」を表す user_id 列を追加する
-- ----------------------------------------------------------------------------
-- ALTER TABLE     : 既存テーブルの構造を変更するSQL
-- ADD COLUMN      : 新しい列を追加する
-- user_id uuid    : 持ち主のユーザーIDを入れる列（型はuuid）
-- REFERENCES auth.users(id) : 「この値は auth.users テーブルの id を指す」という外部キー
--   auth.users は Supabase が認証ユーザーを管理する標準テーブル
-- DEFAULT auth.uid() : INSERT時に省略されたら「今ログイン中の人のID」が自動で入る
-- ============================================================================
ALTER TABLE books
  ADD COLUMN user_id uuid REFERENCES auth.users(id) DEFAULT auth.uid();

-- RLSを有効化（まだなら）。これが無いとポリシーは効かない
ALTER TABLE books ENABLE ROW LEVEL SECURITY;

-- (1) 読み取り(SELECT)ポリシー: 自分のbooksだけ見える
-- CREATE POLICY "ポリシー名" : 新しいルールを作る
-- ON books               : booksテーブルに対するルール
-- FOR SELECT             : SELECT（読み取り）のときに適用
-- USING ( 条件 )         : この条件が真の行だけ「見える」
-- auth.uid() = user_id   : 今ログイン中の人のID と 行の持ち主ID が一致する行だけ
CREATE POLICY "自分の本だけ読める"
  ON books FOR SELECT
  USING (auth.uid() = user_id);

-- (2) 追加(INSERT)ポリシー: 自分名義でしか追加できない
-- FOR INSERT          : INSERT（追加）のときに適用
-- WITH CHECK ( 条件 ) : 追加しようとする行がこの条件を満たすときだけ許可
--   USING ではなく WITH CHECK を使うのがINSERTの作法
CREATE POLICY "自分の本として追加できる"
  ON books FOR INSERT
  WITH CHECK (auth.uid() = user_id);

-- (3) 更新(UPDATE)ポリシー: 自分の本だけ更新できる
-- USING      : 「どの行を更新対象にできるか」（更新前のチェック）
-- WITH CHECK : 「更新後の行が満たすべき条件」（持ち主を他人に書き換えるのを防ぐ）
CREATE POLICY "自分の本だけ更新できる"
  ON books FOR UPDATE
  USING (auth.uid() = user_id)
  WITH CHECK (auth.uid() = user_id);

-- (4) 削除(DELETE)ポリシー: 自分の本だけ削除できる
CREATE POLICY "自分の本だけ削除できる"
  ON books FOR DELETE
  USING (auth.uid() = user_id);
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: そもそも `auth.uid()` とは何か

```sql
auth.uid() = user_id
```

- `auth.uid()` は「**今このリクエストを送ってきたログインユーザーのID**」を返す、Supabaseが用意した関数です。先ほどの `signInWithPassword` でログインした人のIDが、ここに自動的に入ります。
- `user_id` はテーブルの各行に持たせた「その本の持ち主のID」です。
- つまり `auth.uid() = user_id` は「**この行の持ち主は、今アクセスしている本人ですか？**」という質問です。本人の行だけ真（true）になります。
- ログインしていない場合、`auth.uid()` は `null` になり、どの行とも一致しないため、結果として何も見えません（安全側に倒れる）。

> **用語:** **ポリシー** = 「どの行に・誰が・どの操作をしてよいか」を1つにまとめたルール。テーブルに何枚でも貼れる。

---

##### 解説2: 操作ごとにポリシーを分ける（SELECT / INSERT / UPDATE / DELETE）

```sql
CREATE POLICY "自分の本だけ読める"  ON books FOR SELECT USING (auth.uid() = user_id);
CREATE POLICY "自分の本として追加できる" ON books FOR INSERT WITH CHECK (auth.uid() = user_id);
```

- RLS のポリシーは「読む」「追加する」「更新する」「削除する」の**操作ごとに別々に**作るのが基本です。`FOR SELECT` / `FOR INSERT` のように、どの操作向けかを指定します。
- ポリシーを1枚も貼っていない操作は、RLS有効時には**全面的に禁止**されます（「許可リスト方式」）。だから4種類すべてに対してポリシーを用意しています。

---

##### 解説3: `USING` と `WITH CHECK` の違い

```sql
CREATE POLICY "自分の本だけ更新できる"
  ON books FOR UPDATE
  USING (auth.uid() = user_id)       -- どの行を触ってよいか（変更前）
  WITH CHECK (auth.uid() = user_id); -- 変更後の行が満たすべき条件
```

- `USING` は「**今ある行のうち、どれを対象にできるか**」の条件です。読み取りや、更新・削除の「対象選び」に使われます。
- `WITH CHECK` は「**新しく書き込もうとしている値が、ルールを満たしているか**」の条件です。追加（INSERT）や更新後（UPDATE）の値をチェックします。
- UPDATE で両方を書いているのは、「他人の本を勝手に編集できない（USING）」だけでなく「更新のついでに `user_id` を他人のものに書き換えて乗っ取る、を防ぐ（WITH CHECK）」ためです。

> **用語:** **許可リスト方式** = 「明示的に許可したものだけOK、それ以外は全部禁止」という考え方。セキュリティの基本姿勢。

### Storage: 画像やファイルのアップロードと公開URLの取得

> **▼ このコードがやること（先に日本語で）:** 写真やPDFのような**ファイルを Supabase に保存（アップロード）し、そのファイルを表示するためのURLを取得する**例です。テキストデータはテーブル（DB）に保存しますが、画像のような大きなファイルはDBではなく「Storage（ファイル置き場）」に入れるのが定石です。書籍の表紙画像をアップロードして `cover_url` に保存する、といった用途で使います。

```ts
// 前提: Supabaseダッシュボードの Storage で「book-covers」という
//       バケット（ファイルを入れる箱）を作成し、Public（公開）に設定しておく

// file は <input type="file"> でユーザーが選んだファイル（Fileオブジェクト）
async function uploadCover(file: File) {
  // 保存するときのファイル名（パス）。重複しないよう現在時刻を頭につける
  const filePath = `${Date.now()}-${file.name}`;

  // ① ファイルをアップロードする
  // supabase.storage         : ファイル置き場（Storage）を操作する入り口
  // .from('book-covers')     : 'book-covers' という箱（バケット）を選ぶ
  // .upload(保存先パス, ファイル) : 指定パスにファイルを保存する
  const { data, error } = await supabase.storage
    .from('book-covers')
    .upload(filePath, file);

  if (error) {
    console.error('アップロード失敗:', error.message);
    return null;
  }

  // ② 保存したファイルの「公開URL」を取得する
  // .getPublicUrl(パス) : 公開バケット内のファイルにアクセスできるURLを返す
  const { data: urlData } = supabase.storage
    .from('book-covers')
    .getPublicUrl(filePath);

  console.log('画像のURL:', urlData.publicUrl);
  // 例: https://xxxx.supabase.co/storage/v1/object/public/book-covers/171234-cover.png
  return urlData.publicUrl; // このURLを books.cover_url に保存すれば表示できる
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: バケットという「ファイルの箱」

```ts
supabase.storage.from('book-covers')
```

- `supabase.storage` は、テーブル（DB）とは別の「ファイル置き場」を操作する入り口です。
- `.from('book-covers')` の `'book-covers'` は**バケット**の名前です。バケットは「フォルダのような大きな箱」で、用途ごと（表紙画像用、アバター用…）に分けて作ります。バケットはあらかじめダッシュボードで作っておく必要があります。
- バケットには「Public（誰でもURLで見られる）」と「Private（許可された人だけ）」の設定があります。表紙画像のように隠す必要のないものは Public が手軽です。

> **用語:** **バケット** = Storage の中でファイルをまとめて入れておく箱（フォルダのようなもの）。

---

##### 解説2: アップロードと、保存先パスの決め方

```ts
const filePath = `${Date.now()}-${file.name}`;
const { data, error } = await supabase.storage
  .from('book-covers')
  .upload(filePath, file);
```

- `.upload(保存先パス, ファイル)` で、選んだファイルをバケットに保存します。
- `filePath` は「箱の中でのファイル名」です。同じ名前のファイルがあると上書きエラーになるため、`Date.now()`（今の時刻を数値にしたもの）を頭につけて**名前がかぶらないように**しています。
- `file` は、HTML の `<input type="file">` でユーザーが選んだファイルそのものです。

---

##### 解説3: 公開URLを取り出してDBに保存する

```ts
const { data: urlData } = supabase.storage
  .from('book-covers')
  .getPublicUrl(filePath);
// urlData.publicUrl を books.cover_url に保存する
```

- `.getPublicUrl(パス)` は、保存したファイルを**ブラウザで表示するためのURL**を返します。このメソッドは通信を伴わないので `await` は不要です。
- 返ってきた `urlData.publicUrl` を、`books` テーブルの `cover_url` 列に保存しておけば、画面では `<img src={cover_url}>` のように表示できます。
- 「ファイルそのものは Storage に、その**場所を指すURLだけ**を DB に持つ」という分担がポイントです。

### テーブル結合: 関連する別テーブルのデータも一緒に取得する

> **▼ このコードがやること（先に日本語で）:** ある本のデータを取るとき、**その本の「持ち主ユーザーの情報」も一回でまとめて取得する**例です。データは複数のテーブルに分けて保存しますが（本は books、ユーザー情報は profiles…）、画面に出すときは「本のタイトルと、登録した人の名前」を一緒に見せたいことがよくあります。毎回2回問い合わせる代わりに、Supabase の `select` の中で関連先を指定すると1回でまとめて取れます。

```ts
// 前提: books テーブルに user_id（profiles.id を指す外部キー）があり、
//       profiles テーブルに id, username, avatar_url がある状態

// .select('*, profiles(*)') :
//   '*'          → books自身の全カラム
//   profiles(*)  → 関連する profiles テーブルの全カラムも一緒に取得
const { data, error } = await supabase
  .from('books')
  .select('*, profiles(*)');

if (error) {
  console.error('取得失敗:', error.message);
} else {
  console.log(data);
  // 結果のイメージ（各bookの中に profiles が入れ子で入る）:
  // [
  //   {
  //     id: 'book-1',
  //     title: 'ノルウェイの森',
  //     user_id: 'user-AAA',
  //     profiles: { id: 'user-AAA', username: 'taro', avatar_url: null }
  //   },
  //   ...
  // ]
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `select` の中に別テーブル名を書く

```ts
.select('*, profiles(*)')
```

- これまで `select('*')` は「このテーブルの全カラム」でした。ここに `, profiles(*)` を足すと、「**外部キーでつながっている profiles テーブルの全カラムも一緒にちょうだい**」という意味になります。
- `profiles(*)` の `(*)` は「profiles の全カラム」。`profiles(username, avatar_url)` のように、欲しい列だけ指定することもできます。
- これが成立するのは、`books.user_id` が `profiles.id` を指す**外部キー**として設定されているからです。Supabase はその関係を見て「どの行とどの行がつながっているか」を自動で判断します。

> **用語:** **結合（JOIN）** = 関連する複数のテーブルを、共通のID（外部キー）でつなげて1つの結果としてまとめること。

---

##### 解説2: 結果は「入れ子（ネスト）」になって返る

```ts
// book.profiles.username のように、本の中にユーザー情報が入っている
```

- 結合した結果は、SQLの表のように横に広がるのではなく、**JavaScriptのオブジェクトの入れ子**として返ってきます。各 `book` の中に `profiles` というプロパティができ、その中に持ち主の情報が入ります。
- 画面では `book.profiles.username`（登録者名）のように、ドットでたどって取り出せます。
- 「1回の通信で必要な情報をまとめて取る」ことで、表示が速くなり、コードもシンプルになります。

### 高度なフィルタ: or・複数条件・in・range

> **▼ このコードがやること（先に日本語で）:** 「**もっと細かい条件でデータを絞り込む**」ための道具を整理します。本編では `.eq()`（〜と等しい）だけ使いましたが、実際には「AまたはB」「リストのどれかに当てはまる」「上位20件だけ」のように、いろいろな絞り込みが必要になります。検索機能やページめくり（ページネーション）を作るときに必須の道具です。

```ts
// ① 複数条件（AND）: メソッドを並べると「すべて満たす」になる
// .eq('status', 'reading') : status が 'reading'
// .gte('rating', 4)        : rating が 4 以上（gte = greater than or equal）
const { data: reading } = await supabase
  .from('books')
  .select('*')
  .eq('status', 'reading')
  .gte('rating', 4);

// ② OR条件: 「どちらか一方でも満たせばOK」
// .or('文字列') : カンマ区切りで複数条件を並べ、いずれか真ならマッチ
const { data: orResult } = await supabase
  .from('books')
  .select('*')
  .or('status.eq.completed,rating.eq.5'); // 読了済み または 評価5

// ③ in: 「このリストのどれかに当てはまる」
// .in('カラム', [値1, 値2, ...]) : 値のどれかと一致する行
const { data: inResult } = await supabase
  .from('books')
  .select('*')
  .in('status', ['reading', 'completed']); // reading か completed の本

// ④ range: 「何件目から何件目まで」を取り出す（ページめくり用）
// .range(開始index, 終了index) : 0始まりの番号で範囲指定（両端を含む）
const { data: page1 } = await supabase
  .from('books')
  .select('*')
  .order('created_at', { ascending: false })
  .range(0, 19); // 0〜19番 = 最初の20件
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: メソッドを並べると AND（かつ）になる

```ts
.eq('status', 'reading')
.gte('rating', 4)
```

- 絞り込みメソッドを `.` で続けて並べると、「**そのすべてを満たす行**」だけが残ります（AND条件）。上の例は「読書中 **かつ** 評価4以上」です。
- `.gte` は "greater than or equal"（以上）の略です。仲間に `.gt`（より大きい）、`.lte`（以下）、`.lt`（未満）、`.neq`（等しくない）があります。

> **用語:** **AND条件** = 「AもBも両方とも満たす」絞り込み。**OR条件** = 「AかBのどちらか一方でも満たせばよい」絞り込み。

---

##### 解説2: `.or()` で「どちらか一方」を表す

```ts
.or('status.eq.completed,rating.eq.5')
```

- `.or()` は AND とは逆で、「**並べた条件のどれか1つでも満たせばマッチ**」させます。
- 条件の書き方は少し独特で、`カラム名.演算子.値` を**カンマ区切り**でつなぎます。`status.eq.completed` は「status が completed と等しい」、`rating.eq.5` は「rating が 5」を意味します。
- 上の例は「読了済み、**または** 評価が5の本」を取り出します。

---

##### 解説3: `.in()` で「リストのどれか」を表す

```ts
.in('status', ['reading', 'completed'])
```

- `.in('カラム', [値の配列])` は、「そのカラムが、配列の中のどれかと一致する行」を取り出します。
- 上の例は「status が reading **または** completed の本」で、`.or()` でも書けますが、同じカラムで候補が多いときは `.in()` の方がずっと簡潔です。

---

##### 解説4: `.range()` でページめくり（ページネーション）

```ts
.order('created_at', { ascending: false })
.range(0, 19);
```

- `.range(開始, 終了)` は「**何番目から何番目までの行**」を取り出します。番号は0から始まり、両端を含みます。`range(0, 19)` は最初の20件です。
- 次のページは `range(20, 39)`、その次は `range(40, 59)`…と進めれば、「1ページ20件ずつ表示する」機能が作れます。これを**ページネーション**と呼びます。
- 範囲を指定する前に `.order(...)` で**並び順を固定**しておくのが重要です。順番が決まっていないと「何番目」が毎回ばらついてしまうためです。

> **用語:** **ページネーション** = 大量のデータを「1ページ◯件ずつ」に分けて表示し、ページを切り替えられるようにする仕組み。

### upsert: あれば更新・無ければ挿入

> **▼ このコードがやること（先に日本語で）:** 「**そのデータがすでにあれば上書き更新し、無ければ新しく追加する**」という、insert と update を1回でこなす便利な操作です。たとえば「ユーザー設定」や「お気に入り」のように、「初回は新規作成、2回目以降は更新」となるケースで、毎回「存在するか確認してから insert か update を選ぶ」という面倒な分岐を書かずに済みます。

```ts
// upsert = update + insert
// .upsert(データ, { onConflict: 'カラム名' }) :
//   指定カラムの値が既存の行と「かぶったら」→ 更新
//   かぶらなければ                       → 新規挿入
const { data, error } = await supabase
  .from('books')
  .upsert(
    { id: 'book-123', title: 'リーダブルコード', status: 'completed' },
    { onConflict: 'id' } // id がかぶったら更新、なければ挿入
  )
  .select(); // 挿入/更新後の行を返してもらう

if (error) {
  console.error('upsert失敗:', error.message);
} else {
  console.log('保存結果:', data);
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: insert と update を1つにまとめる

```ts
.upsert(
  { id: 'book-123', title: 'リーダブルコード', status: 'completed' },
  { onConflict: 'id' }
)
```

- `.upsert(データ, オプション)` は、渡したデータが既存の行と「ぶつかる」かどうかで、自動的に挿入か更新かを切り替えます。
- `{ onConflict: 'id' }` は「**`id` 列の値がすでにある行とかぶったら、新規追加ではなく更新する**」という指定です。`id` が `'book-123'` の行が既にあれば中身を上書きし、無ければ新しく1行追加します。
- これがないと、同じ `id` を insert しようとしたときに「重複エラー」で失敗します。upsert はそのエラーを「更新」に振り替えてくれる、と考えると分かりやすいです。

> **用語:** **upsert** = update（更新）と insert（挿入）を合わせた造語。「あれば更新・無ければ挿入」を一発でやる操作。**onConflict** = 「どの列の重複を『同じデータ』とみなすか」の指定。

---

##### 解説2: 最後の `.select()` で結果を受け取る

```ts
.upsert(...).select();
```

- `.upsert(...)` だけでは、保存はされても結果のデータは返ってきません。うしろに `.select()` をつけると、**挿入または更新された後の行**を `data` で受け取れます。
- 「保存したらすぐ画面に最新の内容を反映したい」ときに便利です。

### Realtime購読: テーブルの変更をリアルタイムに受け取る

> **▼ このコードがやること（先に日本語で）:** **データベースが変わった瞬間に、自分の画面へ自動で通知を受け取る**仕組みです。普通は「再読み込みボタンを押す」「ページを開き直す」をしないと新しいデータは見えませんが、Realtime を使うと、誰かが本を追加した瞬間に自分の一覧へ即座に反映できます。チャットアプリの新着メッセージや、複数人で同時に使う管理画面などで活躍します。

```ts
// supabase.channel('名前') : リアルタイム通知の受信チャンネルを作る
// .on('postgres_changes', { ... }, コールバック) :
//   DBの変更が起きたら、第3引数の関数を呼んでもらう設定
const channel = supabase
  .channel('books-changes')
  .on(
    'postgres_changes',
    {
      event: '*',        // INSERT/UPDATE/DELETE すべての変更を対象にする
      schema: 'public',  // 対象スキーマ（通常はpublic）
      table: 'books',    // 監視するテーブル名
    },
    (payload) => {
      // payload に「何が起きたか」の情報が入って渡される
      console.log('変更を検知:', payload.eventType); // 'INSERT' など
      console.log('新しい行:', payload.new);          // 追加/更新後のデータ
    }
  )
  .subscribe(); // ① ここで実際に購読（受信）を開始する

// 画面を閉じるときなどは購読を解除して後始末する
// supabase.removeChannel(channel) : このチャンネルの受信を停止する
// （Reactなら useEffect の後始末関数の中で呼ぶ）
// supabase.removeChannel(channel);
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: チャンネルを作り、「何を監視するか」を指定する

```ts
supabase
  .channel('books-changes')
  .on('postgres_changes', { event: '*', schema: 'public', table: 'books' }, (payload) => { ... })
```

- `.channel('books-changes')` は「リアルタイム通知を受け取る専用の窓口」を1つ作ります。名前（`'books-changes'`）は自分で分かりやすいものを付けるだけです。
- `.on('postgres_changes', {条件}, 関数)` で「**books テーブルに変更が起きたら、この関数を呼んで**」と登録します。
  - `event: '*'` は「追加・更新・削除すべて」。特定の操作だけ見たいなら `'INSERT'` などにします。
  - `table: 'books'` で監視対象のテーブルを指定します。
- 変更が起きると、登録した関数に `payload`（変更内容の詳細）が渡されます。`payload.new` に新しいデータ、`payload.eventType` に「INSERT/UPDATE/DELETE のどれか」が入っています。

> **用語:** **購読（subscribe）** = 「変化があったら教えてね」と登録して、通知を受け取り続けること。新聞の定期購読のイメージ。**コールバック** = 「何かが起きたときに呼んでもらう関数」。

---

##### 解説2: subscribe で開始し、removeChannel で後始末する

```ts
.subscribe(); // 受信を開始
// supabase.removeChannel(channel); // 受信を停止
```

- `.subscribe()` を呼んで初めて、実際に通知の受信が始まります。これを忘れると、設定はしたのに何も届きません。
- 受信は「つなぎっぱなし」になるため、画面を閉じるときなどには `supabase.removeChannel(channel)` で**必ず後始末**します。後始末しないと、見えない場所で通信が残り続け、動作が重くなる原因になります。
- React で使う場合は、`useEffect` の中で `subscribe()` し、その**後始末関数（return する関数）**の中で `removeChannel` を呼ぶ、という形が定番です。

> **補足: Realtime は最初はオフ:** Realtime はテーブルごとに有効化が必要です。Supabaseダッシュボードの該当テーブルで「Realtime」をオンにする（または Replication 設定に追加する）と、上のコードで変更が届くようになります。

---

## 9. トラブルシューティング

### 9.1 接続エラー

#### エラー: `FetchError: request to https://xxxxx.supabase.co/rest/v1/books failed`

**原因:** Supabase に接続できない。

**対処法:**

1. **環境変数を確認する**
   ```bash
   # .env.local の内容を確認（ターミナルで実行）
   # cat : ファイルの中身を表示するUnixコマンド
   cat .env.local
   ```
   - `NEXT_PUBLIC_SUPABASE_URL` が正しいか確認
   - `NEXT_PUBLIC_SUPABASE_ANON_KEY` が正しいか確認
   - URL の末尾にスラッシュ `/` がないか確認（不要です）
   - 余分な空白や改行がないか確認

2. **プロジェクトが起動しているか確認**
   - Supabase Dashboard にアクセスして、プロジェクトが「Active」状態か確認
   - 一時停止されている場合は「Restore project」をクリック

3. **開発サーバーを再起動**
   ```bash
   # 環境変数の変更はサーバー再起動が必要
   # Ctrl + C で停止してから
   # npm run dev : Next.jsの開発サーバーを起動するコマンド
   npm run dev
   ```

#### エラー: `TypeError: fetch failed` または `ECONNREFUSED`

**原因:** ネットワーク接続の問題。

**対処法:**

1. インターネットに接続されているか確認
2. VPN を使用している場合は一時的に無効化してみる
3. ファイアウォールが Supabase への接続をブロックしていないか確認

### 9.2 RLS でデータが取得できない

#### 症状: `data` が空配列 `[]` で返ってくる（エラーは出ない）

これは Supabase で最もよくあるハマりポイントです。

**原因:** RLS（Row Level Security）が有効だが、アクセスを許可するポリシーが設定されていない。

**確認手順:**

1. Supabase Dashboard の「Authentication」>「Policies」で books テーブルのポリシーを確認
2. ポリシーが存在するか、`USING (true)` になっているか確認

**対処法:**

```sql
-- 現在のポリシーを確認
-- pg_policies : PostgreSQLが自動で持つ「ポリシー一覧」を見られるシステムビュー
-- WHERE tablename = 'books' : booksテーブルのポリシーだけを表示
SELECT * FROM pg_policies WHERE tablename = 'books';
```

ポリシーが存在しない場合は、セクション5の SQL を再実行してください:

```sql
-- RLS が有効になっているか確認
-- pg_class : テーブル等のメタデータを持つシステムカタログ
-- relname  : 名前（リレーション名）
-- relrowsecurity : RLSが有効か（true/false）
SELECT relname, relrowsecurity
FROM pg_class
WHERE relname = 'books';
-- relrowsecurity が true なら RLS が有効

-- ポリシーがない場合は追加
-- SELECT用ポリシー
CREATE POLICY "Allow public read access"
  ON books
  FOR SELECT
  USING (true);

-- INSERT用ポリシー
CREATE POLICY "Allow public insert access"
  ON books
  FOR INSERT
  WITH CHECK (true);

-- UPDATE用ポリシー
CREATE POLICY "Allow public update access"
  ON books
  FOR UPDATE
  USING (true)
  WITH CHECK (true);

-- DELETE用ポリシー
CREATE POLICY "Allow public delete access"
  ON books
  FOR DELETE
  USING (true);
```

**もう一つの対処法（開発時のみ）:**

一時的に RLS を無効化して、RLS が原因かどうかを切り分けることもできます。

```sql
-- ⚠️ 開発環境でのみ使用すること！
-- DISABLE ROW LEVEL SECURITY : RLSを OFF にする
-- 本番でこれをやると全データが世界に公開される
ALTER TABLE books DISABLE ROW LEVEL SECURITY;
```

これでデータが取得できるようになったら、RLS のポリシー設定に問題があることが確定します。問題を解決したら必ず RLS を再度有効化してください:

```sql
-- 再度RLSを有効化（本番運用前に必須）
ALTER TABLE books ENABLE ROW LEVEL SECURITY;
```

#### 症状: INSERT はできるが SELECT でデータが返らない

**原因:** SELECT のポリシーが設定されていない、または条件が間違っている。

**対処法:** 上記の SELECT ポリシーを確認してください。

### 9.3 型エラー

#### エラー: `Property 'books' does not exist on type 'Database'`

**原因:** TypeScript の型定義ファイルが古いか、生成されていない。

**対処法:**

```bash
# 型定義を再生成
# package.json の gen:types スクリプトを実行
npm run gen:types
```

#### エラー: `Type 'string' is not assignable to type 'number'`

**原因:** Supabase から返されるデータの型と、コードで期待している型が一致していない。

**対処法:**

```typescript
// 自動生成された型を使用する
// '@/types/book' は src/types/book.ts （6.4で作った）
import type { Book } from '@/types/book';

// 悪い例: 手動で型を定義しない
interface Book {
  id: number;  // UUID は string なのに number にしている → 型不一致
  // ...
}

// 良い例: 自動生成された型を使う
// Database['public']['Tables']['books']['Row'] で正しい型が得られる
type Book = Database['public']['Tables']['books']['Row'];
```

#### エラー: `Could not find a declaration file for module '@supabase/supabase-js'`

**原因:** `@supabase/supabase-js` がインストールされていないか、TypeScript の設定に問題がある。

**対処法:**

```bash
# パッケージを再インストール
npm install @supabase/supabase-js

# node_modules を削除して再インストール
# rm -rf : ディレクトリを再帰的・強制的に削除（Unix）
# Windowsの場合は: rmdir /s /q node_modules && del package-lock.json
rm -rf node_modules package-lock.json
npm install
```

### 9.4 SQL エラー

#### エラー: `relation "books" already exists`

**原因:** テーブルがすでに存在する状態で `CREATE TABLE` を実行した。

**対処法:**

```sql
-- テーブルを削除してから再作成（データも消える）
-- DROP TABLE : テーブルを削除するSQL
-- IF EXISTS  : 存在する場合のみ実行（存在しなくてもエラーにならない）
DROP TABLE IF EXISTS books;

-- または、テーブルが存在しない場合のみ作成
-- CREATE TABLE IF NOT EXISTS : 存在しない場合のみ作成
CREATE TABLE IF NOT EXISTS books (
  -- ...
);
```

#### エラー: `new row violates check constraint "books_rating_check"`

**原因:** `rating` カラムに 1〜5 の範囲外の値を挿入しようとした。

**対処法:**

```typescript
// 悪い例: 範囲外の値
const book = { title: 'テスト', author: 'テスト', rating: 10 };  // CHECK違反

// 良い例: 1〜5 の範囲内の値
const book = { title: 'テスト', author: 'テスト', rating: 5 };
```

#### エラー: `new row violates check constraint "books_status_check"`

**原因:** `status` カラムに許可されていない値を挿入しようとした。

**対処法:**

```typescript
// 悪い例: 許可されていない値
const book = { title: 'テスト', author: 'テスト', status: 'done' };  // CHECK違反

// 良い例: 許可された値のいずれか
const book = { title: 'テスト', author: 'テスト', status: 'completed' };
// 許可された値: 'reading' | 'completed' | 'want_to_read'
```

### 9.5 よくある質問

**Q: anon key が漏れたらどうなりますか？**

A: anon key は「公開キー」なので、ブラウザの JavaScript から見える前提で設計されています。RLS を正しく設定していれば、anon key だけではデータに不正アクセスできません。ただし、RLS を無効にしている場合は危険です。本番環境では必ず RLS を有効にしてください。

**Q: service_role key はいつ使いますか？**

A: `service_role` key は RLS をバイパスする管理者用のキーです。**絶対にフロントエンドに含めないでください。** サーバーサイドのバッチ処理やマイグレーションスクリプトでのみ使用します。漏えいすると全データの読み書きが攻撃者に許されるため、Gitリポジトリ・公開URL・ブラウザJSに含めるのは厳禁です。

**Q: テーブルの定義を変更したい場合は？**

A: SQL Editor で `ALTER TABLE` コマンドを使用します。変更後は `npm run gen:types` で型定義を再生成してください。

> **▼ このコードがやること（先に日本語で）:** 作成済みのテーブルに対して、**列を後から追加・削除したり、列の型を変えたりする**SQLの例です。共通して `ALTER TABLE`（テーブル定義を変更する命令）で始まり、`ADD COLUMN`（追加）・`DROP COLUMN`（削除）・`ALTER COLUMN ... TYPE`（型変更）と続けます。`DROP COLUMN` はその列のデータも消える点に注意してください。

```sql
-- カラムの追加
-- ALTER TABLE : テーブル定義の変更
-- ADD COLUMN  : 新しいカラムを追加
-- page_count integer : カラム名と型
ALTER TABLE books ADD COLUMN page_count integer;

-- カラムの削除
-- DROP COLUMN : 既存カラムを削除（データも消える）
ALTER TABLE books DROP COLUMN page_count;

-- カラムの型変更
-- ALTER COLUMN ... TYPE 新しい型 : カラムの型を変える
-- smallint : 小さい整数型（-32768〜32767）
ALTER TABLE books ALTER COLUMN rating TYPE smallint;
```

---

## まとめ

この章で学んだことを振り返りましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- Root -->
  <div style="background: #eff6ff; border: 2px solid #3b82f6; border-radius: 12px; padding: 14px 24px; text-align: center; margin-bottom: 16px; box-shadow: 0 4px 16px rgba(59,130,246,0.12);">
    <div style="font-weight: 700; color: #1e40af; font-size: 16px;">Chapter 5 Summary</div>
  </div>
  <!-- Branches -->
  <div style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center;">
    <!-- Supabase Basics -->
    <div style="flex: 1; min-width: 190px; max-width: 220px; border: 2px solid #f59e0b; border-radius: 10px; overflow: hidden;">
      <div style="background: #fff7ed; padding: 10px 14px; font-weight: 700; color: #92400e; font-size: 13px; border-bottom: 1px solid #fde68a; text-align: center;">Supabase Basics</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">BaaS concept</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">Firebase comparison</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569;">PostgreSQL benefits</div>
    </div>
    <!-- DB Design -->
    <div style="flex: 1; min-width: 190px; max-width: 220px; border: 2px solid #3ecf8e; border-radius: 10px; overflow: hidden;">
      <div style="background: #ecfdf5; padding: 10px 14px; font-weight: 700; color: #065f46; font-size: 13px; border-bottom: 1px solid #a7f3d0; text-align: center;">DB Design</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">Table / Row / Column</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">PK / FK</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">Data types & constraints</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569;">books table design</div>
    </div>
    <!-- Security -->
    <div style="flex: 1; min-width: 190px; max-width: 220px; border: 2px solid #ef4444; border-radius: 10px; overflow: hidden;">
      <div style="background: #fef2f2; padding: 10px 14px; font-weight: 700; color: #991b1b; font-size: 13px; border-bottom: 1px solid #fecaca; text-align: center;">Security</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">RLS concept</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569;">Policy configuration</div>
    </div>
    <!-- Client Setup -->
    <div style="flex: 1; min-width: 190px; max-width: 220px; border: 2px solid #8b5cf6; border-radius: 10px; overflow: hidden;">
      <div style="background: #faf5ff; padding: 10px 14px; font-weight: 700; color: #5b21b6; font-size: 13px; border-bottom: 1px solid #ddd6fe; text-align: center;">Client Setup</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">supabase-js install</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">Env variables</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">Client initialization</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569;">TS type generation</div>
    </div>
    <!-- CRUD -->
    <div style="flex: 1; min-width: 190px; max-width: 220px; border: 2px solid #0d9488; border-radius: 10px; overflow: hidden;">
      <div style="background: #f0fdfa; padding: 10px 14px; font-weight: 700; color: #134e4a; font-size: 13px; border-bottom: 1px solid #99f6e4; text-align: center;">CRUD Operations</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">SELECT (Read)</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">INSERT (Create)</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569; border-bottom: 1px solid #f1f5f9;">UPDATE (Update)</div>
      <div style="padding: 8px 14px; font-size: 12px; color: #475569;">DELETE (Delete)</div>
    </div>
  </div>
</div>

**この章で達成したこと:**

1. Supabase アカウントを作成し、プロジェクトをセットアップした
2. リレーショナルデータベースの基礎を学んだ
3. 書籍管理アプリの `books` テーブルを設計・作成した
4. RLS を設定してセキュリティを確保した
5. Supabase クライアントをセットアップした
6. CRUD の全操作（SELECT / INSERT / UPDATE / DELETE）を学んだ
7. テストデータを投入してデータベースの動作を確認した

**次の章では:**

第6章では、ここで作成した Supabase データベースと React のフロントエンドを接続し、実際に動く書籍管理アプリの画面を構築していきます。
