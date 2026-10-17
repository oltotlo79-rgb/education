//! RustNote のライブラリ部分（アプリ本体）。
//!
//! 実行ファイル（src/main.rs）とテスト（tests/）の両方から使えるように、
//! アプリの中身はすべてライブラリとして書き、ここで部品（モジュール）を公開する。
//!
//! 部品の役割:
//! - config     … 環境変数から設定（Config）を読む
//! - domain     … 検証済みの型（UserId・NoteId・Email・NoteTitle など）
//! - error      … エラーの分類（RepoError・AppError）と HTTP ステータスへの対応
//! - db         … DB への接続とマイグレーション
//! - repository … DB の読み書き（NoteRepository などの trait と libSQL 版の実装）
//! - state      … アプリ全体で共有する状態（AppState）
//! - auth       … パスワードのハッシュ化・ログイン・ログイン中のユーザー
//! - flash      … 「保存しました」などの一度だけ出すメッセージ
//! - views      … 画面の部品（コンポーネント）とエラー画面
//! - routes     … URL ごとの処理（ページ・フォームの受け取り）とルーター

pub mod auth;
pub mod config;
pub mod db;
pub mod domain;
pub mod error;
pub mod flash;
pub mod repository;
pub mod routes;
pub mod state;
pub mod views;
