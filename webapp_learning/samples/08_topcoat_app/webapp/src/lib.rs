//! 依頼・承認アプリ（Rust＋Topcoat 版の練習）。06 の Django＋React 版と、同じことをする
//!
//! | ファイル | 役目 |
//! |---|---|
//! | config.rs | 設定（環境変数）を読む |
//! | state.rs | データベースの接続と、アプリ全体で 1 つの物 |
//! | models.rs | データの形・状態の名前・日時の表し方 |
//! | auth.rs / http_cookie.rs | ログイン・ログアウト・セッション |
//! | workflow.rs | 依頼の状態を変える処理（いちばん大事） |
//! | queries.rs | 画面に出すために読むだけの SQL |
//! | mail.rs | メール（保存が確定してから送る・検証では宛先を差し替える） |
//! | upload.rs / files.rs | 添付ファイルの確かめ・保存・ダウンロード |
//! | drawing.rs | 図面の JSON |
//! | employees.rs | 社員の CSV の取り込み |
//! | jobs.rs | 毎朝 8:00 の通知 |
//! | request_form.rs / views.rs | 画面の部品 |
//! | app.rs / app/ | 画面（URL ごと） |
//! | cli.rs | exe のコマンド |

pub mod app;
pub mod auth;
pub mod cli;
pub mod config;
pub mod drawing;
pub mod employees;
pub mod files;
pub mod http_cookie;
pub mod jobs;
pub mod mail;
pub mod models;
pub mod queries;
pub mod request_form;
pub mod state;
pub mod upload;
pub mod views;
pub mod workflow;
