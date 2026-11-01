//! 画面（ルーティング）。モジュールの並びが、そのまま URL の並びになる
//!
//! | ファイル | URL | 中身 |
//! |---|---|---|
//! | app.rs | `/` | 依頼の一覧（と、全部の画面の外枠） |
//! | app/login.rs | `/login` | ログイン |
//! | app/logout.rs | `/logout` | ログアウト（POST） |
//! | app/requests/new.rs | `/requests/new` | 新しい依頼 |
//! | app/requests/id.rs | `/requests/{id}` | 依頼の詳しい画面と、操作（`/requests/{id}/action`） |
//! | app/requests/id/edit.rs | `/requests/{id}/edit` | 直して出し直す |
//! | app/attachments/id.rs | `/attachments/{id}` | 添付ファイルのダウンロード |
//! | app/approve/token.rs | `/approve/{token}` | メールの URL から開く、承認の画面（ログイン不要） |
//! | app/stats.rs | `/stats` | 集計 |
//! | app/health.rs | `/health` | 動いているかの確かめ（監視用） |

mod approve;
pub mod attachments;
mod health;
pub mod login;
mod logout;
pub mod requests;
mod stats;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    context::Cx,
    cookie::RouterBuilderCookieExt,
    mail::{MailConfig, RouterBuilderMailExt},
    router::{BodyLimit, Router, Slot, StatusCode, error::NotFoundError, href, layout, module_router, page, query_params},
    session::RouterBuilderSessionExt,
    view::{View, error_boundary, view},
};

use crate::{
    auth::{self, current_user, require_user},
    models::fmt_date,
    queries,
    state::{AppState, state},
    views::status_badge,
};

/// ルーターを組み立てる
pub fn router(app_state: AppState, mail: MailConfig) -> Router {
    module_router!()
        .assets(AssetBundle::load().expect("assets フォルダがありません（先に topcoat asset bundle を動かす）"))
        .app_context(app_state) // どのページからも state(cx) で使える
        .mail(mail)
        .cookies()
        .sessions(auth::session_config())
        // 依頼の画面だけ、受け取る大きさの上限を 60MB にする（決まりは 2MB。添付 10MB×5＋図面 5MB が入るように）
        .layer(BodyLimit::max(60 * 1024 * 1024).at("/requests"))
        .build()
}

/// 全部の画面の外枠（ヘッダーとメニュー）
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let user = current_user(cx).await?.clone();
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"依頼・承認アプリ"</title>
                <link rel="stylesheet" href=(asset!("styles.css"))>
                topcoat::dev::script()
            </head>
            <body>
                <header class="site-header">
                    <strong>"依頼・承認アプリ"</strong>
                    if let Some(u) = &user {
                        <nav>
                            <a href=(href!(home))>"依頼の一覧"</a>
                            <a href=(href!(requests::new::new_page))>"新しい依頼"</a>
                            <a href=(href!(stats::stats_page))>"集計"</a>
                            <span>(&u.name) "（" (u.role.label()) "）"</span>
                            <form method="post" action=(href!(logout::logout))>
                                <button type="submit" class="link">"ログアウト"</button>
                            </form>
                        </nav>
                    }
                </header>
                <main>
                    // ページが「見つからない（404）」で終わったら、外枠の中に案内を出す。
                    // ほかのエラー（ログインの画面へ移す、など）は、そのまま外へ投げ直す
                    error_boundary(
                        fallback: |error| {
                            if error.downcast_ref::<NotFoundError>().is_none() {
                                return Err(error);
                            }
                            Ok(view! {
                                (StatusCode::NOT_FOUND)
                                <h1>"見つかりません"</h1>
                                <p>"依頼が無いか、見る権限がありません。" <a href="/">"依頼の一覧"</a> "へ戻ってください。"</p>
                            })
                        },
                        (slot)
                    )
                </main>
            </body>
        </html>
    })
}

/// 一覧の絞り込み（?mine=1 なら、自分が出した依頼だけ）
#[query_params]
struct ListQuery {
    mine: Option<String>,
}

/// 「自分が出した依頼」のリンクに付ける ?mine=1
#[derive(serde::Serialize)]
struct MineOnly {
    mine: &'static str,
}

/// 依頼の一覧（トップの画面）
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let mine = query_params::<ListQuery>(cx).map(|q| q.mine.is_some()).unwrap_or(false);
    let rows = queries::list_requests(&state(cx).pool, &user, mine).await?;
    Ok(view! {
        <h1>"依頼の一覧"</h1>
        <p class="actions left">
            <a href=(href!(home))>"見られる依頼の全部"</a>
            " ｜ "
            <a href=(href!(home).query(MineOnly { mine: "1" }))>"自分が出した依頼"</a>
        </p>
        if rows.is_empty() {
            <p>"依頼はありません。"</p>
        } else {
            <table>
                <thead>
                    <tr><th>"番号"</th><th>"件名"</th><th>"依頼者"</th><th>"希望の納期"</th><th>"状態"</th></tr>
                </thead>
                <tbody>
                    for r in &rows {
                        <tr>
                            <td>(r.id)</td>
                            <td><a href=(href!(requests::id::detail, requests::id::Id(r.id)))>(&r.title)</a></td>
                            <td>(&r.requester_name)</td>
                            <td>(fmt_date(r.due_date))</td>
                            <td>
                                status_badge(status: r.status)
                                if let Some(w) = &r.waiting_for {
                                    " " <small>(w) " さんの承認待ち"</small>
                                }
                            </td>
                        </tr>
                    }
                </tbody>
            </table>
        }
    })
}
