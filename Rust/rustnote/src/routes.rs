//! URL ごとの処理（ルート）と、ルーターの組み立て。
//!
//! Topcoat のモジュールルーティングを使う。`module_router!()` を呼んだこのモジュールが `/` になり、
//! 子モジュールが URL の区切りを1つずつ足していく（ファイルの並び＝URL の並び）。
//!
//! | ファイル | URL |
//! |---|---|
//! | routes.rs                 | GET /（トップ）、全ページ共通のレイアウト、404 の受け皿 |
//! | routes/health.rs          | GET /health |
//! | routes/signup.rs          | GET /signup・POST /signup |
//! | routes/login.rs           | GET /login・POST /login |
//! | routes/logout.rs          | POST /logout |
//! | routes/notes.rs           | GET /notes（?q= 検索・?page= ページ送り）・POST /notes |
//! | routes/notes/id.rs        | GET /notes/{id} |
//! | routes/notes/id/edit.rs   | GET /notes/{id}/edit・POST /notes/{id}/edit |
//! | routes/notes/id/toggle.rs | POST /notes/{id}/toggle |
//! | routes/notes/id/delete.rs | POST /notes/{id}/delete |

pub mod health;
pub mod login;
pub mod logout;
pub mod notes;
pub mod signup;

use topcoat::{
    Result,
    asset::{AssetConfig, RouterBuilderAssetExt},
    context::Cx,
    cookie::RouterBuilderCookieExt,
    router::{
        Body, Next, Router, Slot,
        error::{SeeOther, see_other},
        href, layer, layout, module_router, not_found, page,
        request::{method, uri},
        response::Response,
    },
    session::{RouterBuilderSessionExt, SessionConfig},
    view::{View, error_boundary, view},
};

use crate::auth::current_user;
use crate::flash;
use crate::state::AppState;
use crate::views::{STYLESHEET, error_page, flash_banner, site_header};

/// ルーターを組み立てる。
/// - `state`  … DB とリポジトリ（AppState）
/// - `assets` … CSS などのアセットの配信設定。本番・開発は `AssetBundle::load()`、
///   テストでは CSS の URL だけを決めた設定を渡す
pub fn build_router(state: AppState, assets: impl Into<AssetConfig>) -> Router {
    module_router!()
        // クッキーを読み書きできるようにする（セッションとフラッシュメッセージで使う）
        .cookies()
        // セッション（ログイン）。トークンは __Host-session クッキーで運ばれる
        .sessions(SessionConfig::default())
        // CSS を /_topcoat/assets/... で配信する
        .assets(assets)
        // どのページからも app_state(cx) で借りられるようにする
        .app_context(state)
        .build()
}

// どのルートにも当たらない URL を NotFoundError にして、レイアウトの
// error_boundary（404 の画面）まで届ける
not_found!();

/// アクセスログ: 「GET /notes -> 200 OK (1.2ms)」のように標準エラー出力へ書く
#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    // ヘルスチェックは数十秒ごとに来るので、ログに出さない
    if uri(cx).path() != "/health" {
        let status = match &result {
            Ok(response) => response.status().to_string(),
            // ページが Err(see_other(..)) で返したリダイレクトは、ここではまだ Err に見える
            Err(error) if error.downcast_ref::<SeeOther>().is_some() => "303 See Other".to_string(),
            Err(error) => format!("error ({error})"),
        };
        eprintln!(
            "{} {} -> {} ({:?})",
            method(cx),
            uri(cx).path(),
            status,
            started.elapsed()
        );
    }
    result
}

/// すべてのページを包むレイアウト（HTML の外枠・ヘッダー・エラー画面）
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    // ヘッダーの表示用。DB の失敗でレイアウトごと失敗しないよう、失敗は「未ログイン」扱いにする
    let user = current_user(cx).await.ok().flatten().cloned();

    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"RustNote"</title>
                <link rel="stylesheet" href=(STYLESHEET)>
                // topcoat dev で動かしているときだけ、自動リロード用のスクリプトが入る
                topcoat::dev::script()
            </head>
            <body>
                site_header(user: user)
                <main class="container">
                    // ページで起きたエラーを、ステータスコード付きのエラー画面に変える
                    error_boundary(fallback: |error| error_page(cx, error), (slot))
                </main>
            </body>
        </html>
    })
}

/// GET / : ログイン中ならノート一覧へ、そうでなければ案内を表示する
#[page]
pub async fn home(cx: &Cx) -> Result<impl View> {
    if current_user(cx).await?.is_some() {
        return Err(see_other(href!(notes::index).resolve(cx)).into());
    }
    let flash = flash::take(cx);

    Ok(view! {
        flash_banner(flash: flash)
        <section class="hero">
            <h1>"RustNote"</h1>
            <p>
                "Rust と Topcoat で作った、シンプルなメモアプリです。"
            </p>
            <p class="actions">
                <a class="button" href=(href!(signup::signup_page))>"新規登録"</a>
                <a class="button secondary" href=(href!(login::login_page))>
                    "ログイン"
                </a>
            </p>
        </section>
    })
}
